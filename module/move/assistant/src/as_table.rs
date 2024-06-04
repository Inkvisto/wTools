
use super::*;
use core::ops::{ Deref, DerefMut };
use core::marker::PhantomData;
use core::fmt;

/// Transparent wrapper for table-like structures.
// #[ derive( Debug ) ]
#[ repr( transparent ) ]
pub struct AsTable< T, Row, Cell, Title >( T, ::core::marker::PhantomData< fn () -> ( Row, Title, Cell ) > )
where
  T : TableRows< Row, Cell >,
  T : TableHeader< Title >,
  T : TableSize,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug,
;

impl< T, Row, Cell, Title > AsRef< T > for AsTable< T, Row, Cell, Title >
where
  T : TableRows< Row, Cell > + TableHeader< Title > + TableSize,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug
{
  fn as_ref( &self ) -> &T
  {
    &self.0
  }
}

impl< T, Row, Cell, Title > AsMut< T > for AsTable< T, Row, Cell, Title >
where
  T : TableRows< Row, Cell > + TableHeader< Title > + TableSize,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug
{
  fn as_mut( &mut self ) -> &mut T
  {
    &mut self.0
  }
}

impl< T, Row, Cell, Title > Deref for AsTable< T, Row, Cell, Title >
where
  T : TableRows< Row, Cell > + TableHeader< Title > + TableSize,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug
{
  type Target = T;

  fn deref( &self ) -> &Self::Target
  {
    &self.0
  }
}

impl< T, Row, Cell, Title > DerefMut for AsTable< T, Row, Cell, Title >
where
  T : TableRows< Row, Cell > + TableHeader< Title > + TableSize,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug
{
  fn deref_mut( &mut self ) -> &mut Self::Target
  {
    &mut self.0
  }
}

impl< T, Row, Cell, Title > From< T > for AsTable< T, Row, Cell, Title >
where
  T : TableRows< Row, Cell > + TableHeader< Title > + TableSize,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug
{
  fn from( table : T ) -> Self
  {
    AsTable( table, PhantomData )
  }
}

// impl< T, Row, Cell, Title > From< AsTable< T, Row, Cell, Title > > for T
// where
//   T : TableRows< Row, Cell > + TableHeader< Title > + TableSize,
//   Row : Cells< Cell >,
//   Title : fmt::Debug,
//   Cell : fmt::Debug
// {
//   fn from( as_table : AsTable< T, Row, Cell, Title > ) -> Self
//   {
//     as_table.0
//   }
// }

impl< T, Row, Cell, Title > Default for AsTable< T, Row, Cell, Title >
where
  T : Default + TableRows< Row, Cell > + TableHeader< Title > + TableSize,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug
{
  fn default() -> Self
  {
    AsTable( T::default(), PhantomData )
  }
}

impl< T, Row, Cell, Title > fmt::Debug for AsTable< T, Row, Cell, Title >
where
  T : TableRows< Row, Cell > + TableHeader< Title > + TableSize + fmt::Debug,
  Row : Cells< Cell >,
  Title : fmt::Debug,
  Cell : fmt::Debug
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "AsTable" )
    .field( "0", &self.0 )
    .finish()
  }
}