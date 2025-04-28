// mod singlethread;
mod multithread;
mod queue;

pub(crate) use multithread::Task;

thread_local! {
  static CAN_BLOCK: bool = web_sys::window().is_none();
}
