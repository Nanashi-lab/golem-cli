// Generated by `wit-bindgen` 0.40.0.

pub fn add(value : UInt64) -> Unit {
      @src.state.add(value)
}

pub fn get() -> UInt64 {
      @src.state.value
}

