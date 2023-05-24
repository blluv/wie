use alloc::vec;

use wie_backend::task;

use crate::{
    base::{JavaClassProto, JavaContext, JavaMethodProto, JavaResult},
    method::MethodImpl,
    proxy::JavaObjectProxy,
    JavaError,
};

// class org.kwis.msp.lcdui.Card
pub struct Card {}

impl Card {
    pub fn as_proto() -> JavaClassProto {
        JavaClassProto {
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init),
                JavaMethodProto::new("<init>", "(I)V", Self::init_1),
                JavaMethodProto::new("getWidth", "()I", Self::get_width),
                JavaMethodProto::new("getHeight", "()I", Self::get_height),
            ],
            fields: vec![],
        }
    }

    async fn init(context: &mut dyn JavaContext, instance: JavaObjectProxy) -> JavaResult<()> {
        log::debug!("Card::<init>({:#x})", instance.ptr_instance);

        context.spawn(
            (|_: &mut dyn JavaContext| async {
                loop {
                    task::sleep(16).await;

                    // call self::paint
                }

                #[allow(unreachable_code)]
                Ok::<_, JavaError>(()) // to conveniently specify the return type
            })
            .into_body(),
        )?;

        Ok(())
    }

    async fn init_1(_: &mut dyn JavaContext, instance: JavaObjectProxy, a0: u32) -> JavaResult<()> {
        log::debug!("Card::<init>({:#x}, {})", instance.ptr_instance, a0);

        Ok(())
    }

    async fn get_width(_: &mut dyn JavaContext) -> JavaResult<u32> {
        log::debug!("Card::get_width");

        Ok(320) // TODO: hardcoded
    }

    async fn get_height(_: &mut dyn JavaContext) -> JavaResult<u32> {
        log::debug!("Card::get_height");

        Ok(480) // TODO: hardcoded
    }
}
