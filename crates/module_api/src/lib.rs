#[repr(C)]
pub struct Module {
    pub name: &'static [u8],
    pub authenticate: extern "C" fn(i32) -> u32,
}
