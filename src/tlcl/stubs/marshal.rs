pub fn tpm_marshal_command(
  _command: u32,
  _command_body: *const core::ffi::c_void,
  _buffer: &mut [u8],
  _buffer_size: usize,
) -> i32 {
  -1
}
