use std::marker::PhantomData;

fn main() {
    let cx: &'static Runtime = Box::leak(Box::default());
    let count = create_signal(cx, 0);
}

#[derive(Debug, Default)]
struct Runtime {}

#[derive(Debug, Clone, Copy)]
struct Signal<T> {
    cx: &'static Runtime,
    id: SignalId,
    ty: PhantomData<T>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct SignalId(usize);

fn create_signal<T>(cx: &'static Runtime, value: T) -> Signal<T> {
    todo!();
}
fn create_effect<T>(cx: &'static Runtime, f: impl Fn() + 'static) {
    todo!();
}
