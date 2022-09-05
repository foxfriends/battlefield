use std::cell::RefCell;
use std::rc::Rc;
use yew::use_mut_ref;

pub fn use_mirror_ref<T: 'static>(value: T) -> Rc<RefCell<Rc<T>>> {
    let value = Rc::new(value);
    let mirror = use_mut_ref(|| value.clone());
    *mirror.borrow_mut() = value;
    mirror
}
