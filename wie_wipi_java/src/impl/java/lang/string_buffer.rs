use alloc::{
    string::{String as RustString, ToString},
    vec,
    vec::Vec,
};

use bytemuck::cast_slice;

use crate::{
    array::Array,
    base::{JavaClassProto, JavaFieldProto, JavaMethodFlag, JavaMethodProto, JavaWord},
    r#impl::java::lang::String,
    JavaContext, JavaObjectProxy, JavaResult,
};

// class java.lang.StringBuffer
pub struct StringBuffer {}

impl StringBuffer {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, JavaMethodFlag::NONE),
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init_with_string, JavaMethodFlag::NONE),
                JavaMethodProto::new(
                    "append",
                    "(Ljava/lang/String;)Ljava/lang/StringBuffer;",
                    Self::append_string,
                    JavaMethodFlag::NONE,
                ),
                JavaMethodProto::new("append", "(I)Ljava/lang/StringBuffer;", Self::append_integer, JavaMethodFlag::NONE),
                JavaMethodProto::new("append", "(C)Ljava/lang/StringBuffer;", Self::append_character, JavaMethodFlag::NONE),
                JavaMethodProto::new("toString", "()Ljava/lang/String;", Self::to_string, JavaMethodFlag::NONE),
            ],
            fields: vec![
                JavaFieldProto::new("value", "[C", crate::JavaFieldAccessFlag::NONE),
                JavaFieldProto::new("count", "I", crate::JavaFieldAccessFlag::NONE),
            ],
        }
    }

    async fn init(context: &mut dyn JavaContext, this: JavaObjectProxy<StringBuffer>) -> JavaResult<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:#x})", this.ptr_instance);

        let java_value_array = context.instantiate_array("C", 16).await?;
        context.put_field(&this.cast(), "value", java_value_array.ptr_instance)?;
        context.put_field(&this.cast(), "count", 0)?;

        Ok(())
    }

    async fn init_with_string(context: &mut dyn JavaContext, this: JavaObjectProxy<StringBuffer>, string: JavaObjectProxy<String>) -> JavaResult<()> {
        tracing::debug!("java.lang.StringBuffer::<init>({:#x}, {:#x})", this.ptr_instance, string.ptr_instance);

        let value_array = JavaObjectProxy::new(context.get_field(&string.cast(), "value")?);
        let length = context.array_length(&value_array)?;

        let java_value_array = context.instantiate_array("C", length).await?;
        let src = context.load_array_i16(&value_array, 0, length)?;
        context.store_array_i16(&java_value_array, 0, &src)?;

        context.put_field(&this.cast(), "value", java_value_array.ptr_instance)?;
        context.put_field(&this.cast(), "count", length)?;

        Ok(())
    }

    async fn append_string(
        context: &mut dyn JavaContext,
        this: JavaObjectProxy<StringBuffer>,
        string: JavaObjectProxy<String>,
    ) -> JavaResult<JavaObjectProxy<StringBuffer>> {
        tracing::debug!("java.lang.StringBuffer::append({:#x}, {:#x})", this.ptr_instance, string.ptr_instance);

        let string = String::to_rust_string(context, &string)?;

        Self::append(context, &this, &string).await?;

        Ok(this)
    }

    async fn append_integer(
        context: &mut dyn JavaContext,
        this: JavaObjectProxy<StringBuffer>,
        value: i32,
    ) -> JavaResult<JavaObjectProxy<StringBuffer>> {
        tracing::debug!("java.lang.StringBuffer::append({:#x}, {:#x})", this.ptr_instance, value);

        let digits = value.to_string();

        Self::append(context, &this, &digits).await?;

        Ok(this)
    }

    async fn append_character(
        context: &mut dyn JavaContext,
        this: JavaObjectProxy<StringBuffer>,
        value: i32,
    ) -> JavaResult<JavaObjectProxy<StringBuffer>> {
        tracing::debug!("java.lang.StringBuffer::append({:#x}, {:#x})", this.ptr_instance, value);

        let value = RustString::from_utf16(&[value as u16])?;

        Self::append(context, &this, &value).await?;

        Ok(this)
    }

    async fn to_string(context: &mut dyn JavaContext, this: JavaObjectProxy<StringBuffer>) -> JavaResult<JavaObjectProxy<String>> {
        tracing::debug!("java.lang.StringBuffer::toString({:#x})", this.ptr_instance);

        let java_value = JavaObjectProxy::<Array>::new(context.get_field(&this.cast(), "value")?);
        let count = context.get_field(&this.cast(), "count")?;

        let string = context.instantiate("Ljava/lang/String;").await?.cast();
        context
            .call_method(&string.cast(), "<init>", "([CII)V", &[java_value.ptr_instance, 0, count])
            .await?;

        Ok(string)
    }

    async fn ensure_capacity(context: &mut dyn JavaContext, this: &JavaObjectProxy<StringBuffer>, capacity: JavaWord) -> JavaResult<()> {
        let java_value_array = JavaObjectProxy::new(context.get_field(&this.cast(), "value")?);
        let current_capacity = context.array_length(&java_value_array)?;

        if current_capacity < capacity {
            let old_values = context.load_array_i16(&java_value_array, 0, current_capacity)?;
            let new_capacity = capacity * 2;

            let java_new_value_array = context.instantiate_array("C", new_capacity).await?;
            context.put_field(&this.cast(), "value", java_new_value_array.ptr_instance)?;
            context.store_array_i16(&java_new_value_array, 0, &old_values)?;
            context.destroy(java_value_array.cast())?;
        }

        Ok(())
    }

    async fn append(context: &mut dyn JavaContext, this: &JavaObjectProxy<StringBuffer>, string: &str) -> JavaResult<()> {
        let current_count = context.get_field(&this.cast(), "count")?;

        let value_to_add = string.encode_utf16().collect::<Vec<_>>();
        let count_to_add = value_to_add.len();

        StringBuffer::ensure_capacity(context, this, current_count + count_to_add).await?;

        let java_value_array = JavaObjectProxy::new(context.get_field(&this.cast(), "value")?);
        context.store_array_i16(&java_value_array, current_count, cast_slice(&value_to_add))?;
        context.put_field(&this.cast(), "count", current_count + count_to_add)?;

        Ok(())
    }
}
