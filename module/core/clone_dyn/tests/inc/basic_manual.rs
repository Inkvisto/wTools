
#[ allow( unused_imports ) ]
use super::*;

trait Trait1
where
  Self : clone_dyn::CloneDyn,
{
  fn val( &self ) -> i32;
}

//

impl Trait1 for i32
{
  fn val( &self ) -> i32
  {
    *self
  }
}

impl Trait1 for i64
{
  fn val( &self ) -> i32
  {
    self.clone().try_into().unwrap()
  }
}

impl Trait1 for String
{
  fn val( &self ) -> i32
  {
    self.len().try_into().unwrap()
  }
}

impl< T > Trait1 for &[ T ]
where
  T : clone_dyn::CloneDyn,
{
  fn val( &self ) -> i32
  {
    self.len().try_into().unwrap()
  }
}

impl Trait1 for &str
{
  fn val( &self ) -> i32
  {
    self.len().try_into().unwrap()
  }
}

// == begin of generated

#[ allow( non_local_definitions ) ]
impl < 'c > Clone
for Box< dyn Trait1 + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self
  {
    // let x = &**self;
    // inspect_type::inspect_type_of!( x );
    // clone_dyn::clone( self )
    clone_dyn::clone_into_box( &**self )
  }
}

#[ allow( non_local_definitions ) ]
impl < 'c > Clone
for Box< dyn Trait1 + Send + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
}

#[ allow( non_local_definitions ) ]
impl < 'c > Clone
for Box< dyn Trait1 + Sync + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
}

#[ allow( non_local_definitions ) ]
impl < 'c > Clone
for Box< dyn Trait1 + Send + Sync + 'c >
{
  #[ inline ]
  fn clone( &self ) -> Self { clone_dyn::clone_into_box( &**self ) }
}

// == end of generated

include!( "./only_test/basic.rs" );