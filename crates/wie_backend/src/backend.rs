pub mod canvas;
mod window;

use alloc::{collections::VecDeque, rc::Rc, string::String};
use core::cell::{Ref, RefCell, RefMut};

use wie_base::{Event, Module};

use crate::{executor::Executor, time::Time};

use self::{
    canvas::{ArgbPixel, Canvas, Image, ImageBuffer},
    window::Window,
};

pub struct Backend {
    resource: Rc<RefCell<Resource>>,
    time: Rc<RefCell<Time>>,
    screen_canvas: Rc<RefCell<Box<dyn Canvas>>>,
    events: Rc<RefCell<VecDeque<Event>>>,
    window: Rc<RefCell<Window>>,
}

impl Default for Backend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend {
    pub fn new() -> Self {
        let canvas = ImageBuffer::<ArgbPixel>::new(240, 320); // TODO hardcoded size
        let window = Window::new(canvas.width(), canvas.height());

        Self {
            resource: Rc::new(RefCell::new(Resource::new())),
            time: Rc::new(RefCell::new(Time::new())),
            screen_canvas: Rc::new(RefCell::new(Box::new(canvas))),
            events: Rc::new(RefCell::new(VecDeque::new())),
            window: Rc::new(RefCell::new(window)),
        }
    }

    pub fn resource(&self) -> Ref<'_, Resource> {
        (*self.resource).borrow()
    }

    pub fn time(&self) -> Ref<'_, Time> {
        (*self.time).borrow()
    }

    pub fn screen_canvas(&self) -> RefMut<'_, Box<dyn Canvas>> {
        (*self.screen_canvas).borrow_mut()
    }

    pub fn window(&self) -> RefMut<'_, Window> {
        (*self.window).borrow_mut()
    }

    pub fn pop_event(&self) -> Option<Event> {
        (*self.events).borrow_mut().pop_front()
    }

    pub fn add_resource(&self, path: &str, data: Vec<u8>) {
        (*self.resource).borrow_mut().add(path, data)
    }

    pub fn repaint(&self) {
        let canvas = self.screen_canvas();
        let data = canvas
            .colors()
            .iter()
            .map(|x| ((x.a as u32) << 24) | ((x.r as u32) << 16) | ((x.g as u32) << 8) | (x.b as u32))
            .collect::<Vec<_>>();

        self.window().paint(&data);
    }

    pub fn run<M>(self, mut module: M) -> anyhow::Result<()>
    where
        M: Module + 'static,
    {
        let mut executor = Executor::new();

        module.start()?;

        Window::run(self.window.clone(), move |event| {
            match event {
                Event::Update => executor.tick(&self.time()).map_err(|x| {
                    let dump = module.crash_dump();

                    anyhow::anyhow!("{}\n{}", x, dump)
                })?,
                _ => self.events.borrow_mut().push_back(event),
            }

            Ok::<_, anyhow::Error>(())
        })
    }
}

impl Clone for Backend {
    fn clone(&self) -> Self {
        Self {
            resource: self.resource.clone(),
            time: self.time.clone(),
            screen_canvas: self.screen_canvas.clone(),
            events: self.events.clone(),
            window: self.window.clone(),
        }
    }
}

pub struct Resource {
    files: Vec<(String, Vec<u8>)>,
}

impl Default for Resource {
    fn default() -> Self {
        Self::new()
    }
}

impl Resource {
    pub fn new() -> Self {
        Self { files: Vec::new() }
    }

    pub fn add(&mut self, path: &str, data: Vec<u8>) {
        tracing::debug!("Adding resource {}, {}b", path, data.len());

        self.files.push((path.to_string(), data));
    }

    pub fn id(&self, path: &str) -> Option<u32> {
        tracing::trace!("Looking for resource {}", path);

        for (id, file) in self.files.iter().enumerate() {
            if file.0 == path {
                return Some(id as _);
            }
        }

        tracing::warn!("No such resource {}", path);

        None
    }

    pub fn size(&self, id: u32) -> u32 {
        self.files[id as usize].1.len() as _
    }

    pub fn data(&self, id: u32) -> &[u8] {
        &self.files[id as usize].1
    }
}
