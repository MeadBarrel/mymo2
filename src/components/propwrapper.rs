use std::rc::Rc;
use std::cell::Cell;
use super::{Component, PropComponent};

pub struct PropWrapper<T, C> 
{
    component: C,
    value: Rc<Cell<T>>,
}

impl<T, C> PropWrapper<T, C> {
    pub fn new(component: C, value: Rc<Cell<T>>) -> Self {
        Self {
            component, value
        }
    }
}

impl<T, C> Component for PropWrapper<T, C> 
    where
        T: Default,
        C: PropComponent<Item = T>,
{
    fn add(&mut self, frame: &mut eframe::Frame, ui: &mut eframe::egui::Ui) {
        let mut item = self.value.take();
        self.component.add(frame, ui, &mut item);
        self.value.set(item);
    }
}