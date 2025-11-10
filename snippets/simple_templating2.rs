fn try_querydb( db: &mut DbContext ) -> Option<String> {
  match db.read() {
    Option::Some( data ) => {
      let txt = String::with_capacity( data.len() );
      txt.clone_from(&data);
      Option::Some(txt)
    },
    _ => Option::None,
  }
}