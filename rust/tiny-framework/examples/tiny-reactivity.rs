// based on: https://www.youtube.com/watch?v=GWB3vTWeLd4

use std::{
    any::Any,
    cell::{Cell, RefCell},
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

fn main() {
    let cx: &'static Runtime = Box::leak(Box::default());
    let value = create_signal(cx, 0);
    let twice = move || 2 * value.get();
    create_effect(cx, move || println!("value changed: {}", value.get()));
    create_effect(cx, move || println!("twice changed: {}", twice()));
    value.set(1);
    value.set(2);
}

#[derive(Default)]
struct Runtime {
    signal_values: RefCell<Vec<Box<RefCell<dyn Any>>>>,
    running_effect: Cell<Option<EffectId>>,
    signal_subscribers: RefCell<HashMap<SignalId, HashSet<EffectId>>>,
    effects: RefCell<Vec<Box<dyn Fn()>>>,
}

fn create_signal<T>(cx: &'static Runtime, value: T) -> Signal<T>
where
    T: Clone + 'static,
{
    cx.signal_values
        .borrow_mut()
        .push(Box::new(RefCell::new(value)));
    let id = SignalId(cx.signal_values.borrow().len() - 1);
    Signal {
        cx: cx,
        id,
        ty: PhantomData,
    }
}

fn create_effect(cx: &'static Runtime, f: impl Fn() + 'static) {
    cx.effects.borrow_mut().push(Box::new(f));
    run_effect(cx, EffectId(cx.effects.borrow().len() - 1));
}

fn run_effect(cx: &'static Runtime, effect_id: EffectId) {
    let prev_effect = cx.running_effect.replace(Some(effect_id));
    cx.effects.borrow()[effect_id.0]();
    cx.running_effect.set(prev_effect);
}

#[derive(Clone, Copy)]
struct Signal<T>
where
    T: Clone + 'static,
{
    cx: &'static Runtime,
    id: SignalId,
    ty: PhantomData<T>,
}

impl<T> Signal<T>
where
    T: Clone + 'static,
{
    fn get(&self) -> T {
        if let Some(running_effect) = self.cx.running_effect.get() {
            self.cx
                .signal_subscribers
                .borrow_mut()
                .entry(self.id)
                .or_default()
                .insert(running_effect);
        }
        self.cx.signal_values.borrow()[self.id.0]
            .borrow()
            .downcast_ref::<T>()
            .unwrap()
            .clone()
    }
    fn set(&self, value: T) {
        *self.cx.signal_values.borrow()[self.id.0]
            .borrow_mut()
            .downcast_mut::<T>()
            .unwrap() = value;
        let maybe_subscribers = self.cx.signal_subscribers.borrow().get(&self.id).cloned();
        if let Some(subscribers) = maybe_subscribers {
            subscribers
                .iter()
                .for_each(|&effect_id| run_effect(self.cx, effect_id))
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SignalId(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct EffectId(usize);
