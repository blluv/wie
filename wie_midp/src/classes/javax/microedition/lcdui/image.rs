use alloc::vec;

use java_class_proto::JavaMethodProto;
use java_constants::MethodAccessFlags;
use java_runtime::classes::java::lang::String;
use jvm::{Array, ClassInstanceRef, Jvm, Result as JvmResult};

use wie_jvm_support::{WieJavaClassProto, WieJvmContext};

use crate::classes::javax::microedition::lcdui::Graphics;

// class javax.microedition.lcdui.Image
pub struct Image;

impl Image {
    pub fn as_proto() -> WieJavaClassProto {
        WieJavaClassProto {
            name: "javax/microedition/lcdui/Image",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("getWidth", "()I", Self::get_width, Default::default()),
                JavaMethodProto::new("getHeight", "()I", Self::get_height, Default::default()),
                JavaMethodProto::new(
                    "getGraphics",
                    "()Ljavax/microedition/lcdui/Graphics;",
                    Self::get_graphics,
                    Default::default(),
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(II)Ljavax/microedition/lcdui/Image;",
                    Self::create_image,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "([BII)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_from_data,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new(
                    "createImage",
                    "(Ljava/lang/String;)Ljavax/microedition/lcdui/Image;",
                    Self::create_image_from_name,
                    MethodAccessFlags::STATIC,
                ),
            ],
            fields: vec![],
        }
    }

    async fn init(jvm: &Jvm, _context: &mut WieJvmContext, this: ClassInstanceRef<Self>) -> JvmResult<()> {
        tracing::debug!("javax.microedition.lcdui.Image::<init>({:?})", &this);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        Ok(())
    }

    async fn get_width(_jvm: &Jvm, _context: &mut WieJvmContext, this: ClassInstanceRef<Self>) -> JvmResult<i32> {
        tracing::warn!("stub javax.microedition.lcdui.Image::getWidth({:?})", &this);

        Ok(100)
    }

    async fn get_height(_jvm: &Jvm, _context: &mut WieJvmContext, this: ClassInstanceRef<Self>) -> JvmResult<i32> {
        tracing::warn!("stub javax.microedition.lcdui.Image::getHeight({:?})", &this);

        Ok(100)
    }

    async fn get_graphics(jvm: &Jvm, _context: &mut WieJvmContext, this: ClassInstanceRef<Self>) -> JvmResult<ClassInstanceRef<Graphics>> {
        tracing::warn!("stub javax.microedition.lcdui.Image::getGraphics({:?})", &this);

        let graphics = jvm.new_class("javax/microedition/lcdui/Graphics", "()V", ()).await?;

        Ok(graphics.into())
    }

    async fn create_image(jvm: &Jvm, _context: &mut WieJvmContext, width: i32, height: i32) -> JvmResult<ClassInstanceRef<Self>> {
        tracing::warn!("stub javax.microedition.lcdui.Image::createImage({}, {})", width, height);

        let image = jvm.new_class("javax/microedition/lcdui/Image", "()V", ()).await?;

        Ok(image.into())
    }

    async fn create_image_from_data(
        jvm: &Jvm,
        _context: &mut WieJvmContext,
        data: ClassInstanceRef<Array<i8>>,
        offset: i32,
        length: i32,
    ) -> JvmResult<ClassInstanceRef<Self>> {
        tracing::warn!("stub javax.microedition.lcdui.Image::createImage({:?}, {}, {})", data, offset, length);

        let image = jvm.new_class("javax/microedition/lcdui/Image", "()V", ()).await?;

        Ok(image.into())
    }

    async fn create_image_from_name(jvm: &Jvm, _context: &mut WieJvmContext, name: ClassInstanceRef<String>) -> JvmResult<ClassInstanceRef<Self>> {
        tracing::warn!("stub javax.microedition.lcdui.Image::createImage({:?})", name);

        let image = jvm.new_class("javax/microedition/lcdui/Image", "()V", ()).await?;

        Ok(image.into())
    }
}
