#[ allow( unused_imports ) ]
use super::*;

use assistant::
{
  Fields,
  IteratorTrait,
  AsTable,
  Cells,
  TableSize,
  TableRows,
  TableHeader,
  TableFormatter,
};

use std::
{
  collections::HashMap,
  borrow::Cow,
};

/// Struct representing a test object with various fields.
#[ derive( Clone, Debug ) ]
pub struct TestObject
{
  pub id : String,
  pub created_at : i64,
  pub file_ids : Vec< String >,
  pub tools : Option< Vec< HashMap< String, String > > >,
}

impl< 'a > Fields< 'a, &'static str, String > for TestObject
{
  fn fields( &'a self ) -> impl IteratorTrait< Item = ( &'static str, Cow< 'a, String > ) >
  {
    let mut vec : Vec< ( &'static str, Cow< 'a, String > ) > = Vec::new();

    vec.push( ( "id", Cow::Borrowed( &self.id ) ) );
    vec.push( ( "created_at", Cow::Owned( self.created_at.to_string() ) ) );
    vec.push( ( "file_ids", Cow::Owned( format!( "{:?}", self.file_ids ) ) ) );

    if let Some( tools ) = &self.tools
    {
      vec.push( ( "tools", Cow::Owned( format!( "{:?}", tools ) ) ) );
    }

    vec.into_iter()
  }
}

impl< 'a > Fields< 'a, usize, TestObject > for Vec< TestObject >
{
  fn fields( &'a self ) -> impl IteratorTrait< Item = ( usize, Cow< 'a, TestObject > ) >
  {
    self.iter().enumerate().map( | ( key, val ) | ( key, Cow::Borrowed( val ) ) )
  }
}

//

#[ test ]
fn test_table_to_string()
where
  for< 'a > AsTable< 'a, Vec< TestObject >, usize, TestObject, &'static str, String, &'static str > : TableFormatter< 'a >,
{
  use the_module::TableToString;

  let test_objects = vec!
  [
    TestObject
    {
      id : "1".to_string(),
      created_at : 1627845583,
      file_ids : vec![ "file1".to_string(), "file2".to_string() ],
      tools : Some( vec!
      [
        {
          let mut map = HashMap::new();
          map.insert( "tool1".to_string(), "value1".to_string() );
          map
        },
        {
          let mut map = HashMap::new();
          map.insert( "tool2".to_string(), "value2".to_string() );
          map
        }
      ]),
    },
    TestObject
    {
      id : "2".to_string(),
      created_at : 1627845584,
      file_ids : vec![ "file3".to_string(), "file4".to_string() ],
      tools : None,
    }
  ];

  let cells = Cells::< '_, &'static str, String >::cells( &test_objects[ 0 ] );
  assert_eq!( cells.len(), 4 );
  // dbg!( cells.collect::< Vec< _ > >() );
  drop( cells );

  let as_table : AsTable< '_, Vec< TestObject >, usize, TestObject, &str, String, &str > = AsTable::new( &test_objects );
  let size = TableSize::< '_ >::table_size( &as_table );
  assert_eq!( size, [ 2, 4 ] );
  let rows = TableRows::rows( &as_table );
  assert_eq!( rows.len(), 2 );
  // dbg!( rows.collect::< Vec< _ > >() );
  let header = TableHeader::header( &as_table );
  assert!( header.is_some() );
  let header = header.unwrap();
  assert_eq!( header.len(), 4 );
  assert_eq!( header.collect::< Vec< _ > >(), vec![ ( "id", "id" ), ( "created_at", "created_at" ), ( "file_ids", "file_ids" ), ( "tools", "tools" ) ] );
  // dbg!( header.collect::< Vec< _ > >() );

  let mut output = String::new();
  let mut formatter = the_module::Formatter::new( &mut output, Default::default() );
  let got = the_module::TableFormatter::fmt( &as_table, &mut formatter );
  assert!( got.is_ok() );
  println!( "{}", &output );

  // use the_module::{ TableFormatter, TableToString };
  // AsTable::new( test_objects );
  // // let table_string = AsTable::new( test_objects ).table_to_string();
  let table_string = as_table.table_to_string();
  // println!( "{}", table_string );
  assert!( table_string.contains( "id" ) );
  assert!( table_string.contains( "created_at" ) );
  assert!( table_string.contains( "file_ids" ) );
  assert!( table_string.contains( "tools" ) );

}
