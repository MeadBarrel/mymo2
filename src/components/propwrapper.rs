use std::{rc::Rc, marker::PhantomData};
use std::cell::Cell;
use super::{AppComponent, PropComponent};

pub struct PropWrapper<T, C, A> 
{
    component: C,
    value: Rc<Cell<T>>,
    _context: PhantomData<A>
}

impl<T, C, A> PropWrapper<T, C, A> {
    pub fn new(component: C, value: Rc<Cell<T>>) -> Self {
        Self {
            component, value, _context: PhantomData
        }
    }
}

impl<T, C, A> AppComponent for PropWrapper<T, C, A> 
    where
        T: Default,
        C: PropComponent<Item = T, Context = A>,
{
    type Context = A;

    fn add(&mut self, ctx: &mut Self::Context, ui: &mut eframe::egui::Ui) {
        let mut item = self.value.take();
        self.component.add(ctx, ui, &mut item);
        self.value.set(item);
    }
}