use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

pub struct FrameScheduler {
    inner: Rc<RefCell<SchedulerInner>>,
}

pub struct Stopper {
    inner: Rc<RefCell<SchedulerInner>>,
}

struct SchedulerInner {
    handle: Option<i32>,
    callback: Option<Box<dyn FnMut(f64)>>,
    last_ts: f64,
}

impl FrameScheduler {
    pub fn new() -> Self {
        Self {
            inner: Rc::new(RefCell::new(SchedulerInner {
                handle: None,
                callback: None,
                last_ts: 0.0,
            })),
        }
    }

    pub fn start<F: FnMut(f64) + 'static>(&mut self, cb: F) -> Stopper {
        {
            let mut i = self.inner.borrow_mut();
            i.callback = Some(Box::new(cb));
            i.last_ts = 0.0;
        }
        Self::schedule(self.inner.clone());
        Stopper {
            inner: self.inner.clone(),
        }
    }

    pub fn stop(&mut self) {
        stop_inner(&self.inner);
    }

    fn schedule(inner: Rc<RefCell<SchedulerInner>>) {
        let inner_cb = inner.clone();
        let closure = Closure::once_into_js(move |ts: f64| {
            let should_continue = {
                let mut b = inner_cb.borrow_mut();
                b.handle = None;
                let dt = if b.last_ts == 0.0 { 16.0 } else { ts - b.last_ts };
                b.last_ts = ts;
                if let Some(cb) = b.callback.as_mut() {
                    cb(dt);
                }
                b.callback.is_some()
            };
            if should_continue {
                FrameScheduler::schedule(inner_cb);
            }
        });
        let h = web_sys::window()
            .unwrap()
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .ok();
        inner.borrow_mut().handle = h;
    }
}

impl Default for FrameScheduler {
    fn default() -> Self {
        Self::new()
    }
}

fn stop_inner(inner: &Rc<RefCell<SchedulerInner>>) {
    let mut i = inner.borrow_mut();
    if let Some(h) = i.handle.take() {
        web_sys::window().unwrap().cancel_animation_frame(h).ok();
    }
    i.callback = None;
}

impl Drop for Stopper {
    fn drop(&mut self) {
        stop_inner(&self.inner);
    }
}
