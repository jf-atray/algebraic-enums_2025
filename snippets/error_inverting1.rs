Result<T, E> {
  Ok(T),
  Err(E),
}

enum ListenErrors { 
  PipePanic,
  MangledData,
  BusinessLogic,
}

struct Customer;
impl Customer {
  pub fn try_new() -> Option<Self> {
    //..
  }
}
