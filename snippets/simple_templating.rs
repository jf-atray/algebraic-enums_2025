// Generics in sum-types is the magic.
// This example commonly handles "nullable" types
enum Option<T> {
  Some<T>,
  None,
}

fn handle( &mut self, user_input: Option<String> ) {
  if let Option::Some( text ) = user_input {
    // without cheating, I'm never allowed to
    // handle an _actually_ null value!
    self.submit(text);
  }
}