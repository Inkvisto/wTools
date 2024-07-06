//!
//! It's often necessary to wrap something inot a local structure and this file contains wrapper of `Option< Cow< 'a, T > >`.
//!

use core::fmt;
use std::borrow::Cow;
use core::ops::{ Deref };

// xxx : review
/// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
pub trait IntoMaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  // T : Clone,

{
  /// Converter into universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
  fn into_maybe_as( self ) -> MaybeAs< 'a, T, Marker >;
}

impl< 'a, T, Marker > IntoMaybeAs< 'a, T, Marker > for < T as std::borrow::ToOwned >::Owned
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  // T : Clone,
{
  #[ inline( always ) ]
  fn into_maybe_as( self ) -> MaybeAs< 'a, T, Marker >
  {
    MaybeAs::< 'a, T, Marker >::new( self )
  }
}

// impl< 'a, T, Marker > IntoMaybeAs< 'a, T, Marker > for &'a T
// where
//   T : std::borrow::ToOwned + ?Sized,
//   < T as std::borrow::ToOwned >::Owned : Clone,
//   // T : Clone,
//
// {
//   #[ inline( always ) ]
//   fn into_maybe_as( self ) -> MaybeAs< 'a, T, Marker >
//   {
//     MaybeAs::< 'a, T, Marker >::new_with_ref( self )
//   }
// }

// xxx
// impl< 'a, T, Marker > IntoMaybeAs< 'a, T, Marker > for ()
// where
//   T : Clone,
// {
//   #[ inline( always ) ]
//   fn into_maybe_as( self ) -> MaybeAs< 'a, T, Marker >
//   {
//     MaybeAs::< 'a, T, Marker >( None )
//   }
// }

/// Universal wrapper with transparent option of copy on write reference emphasizing a specific aspect of identity of its internal type.
#[ repr( transparent ) ]
// #[ derive( Clone ) ]
pub struct MaybeAs< 'a, T, Marker >( pub Option< Cow< 'a, T > >, ::core::marker::PhantomData< fn() -> Marker > )
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  // T : Clone,
;

impl< 'a, T, Marker > Clone for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
{
  fn clone( &self ) -> Self
  {
    Self( self.0.clone(), ::core::marker::PhantomData )
  }
}

impl< 'a, T, Marker > MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
{

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn none() -> Self
  {
    Self( None, ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new( src : < T as std::borrow::ToOwned >::Owned ) -> Self
  {
    Self( Some( Cow::Owned( src ) ), ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new_with_ref( src : &'a T ) -> Self
  {
    Self( Some( Cow::Borrowed( src ) ), ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn new_with_inner( src : Option< Cow< 'a, T > > ) -> Self
  {
    Self( src, ::core::marker::PhantomData )
  }

  /// Just a constructor.
  #[ inline( always ) ]
  pub fn inner( self ) -> Option< Cow< 'a, T > >
  {
    self.0
  }

}

impl< 'a, T, Marker > AsRef< Option< Cow< 'a, T > > > for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  // T : Clone,

  Self : 'a,
{
  fn as_ref( &self ) -> &Option< Cow< 'a, T > >
  {
    &self.0
  }
}

impl< 'a, T, Marker > Deref for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  // T : Clone,

  Marker : 'static,
{
  type Target = Option< Cow< 'a, T > >;
  fn deref( &self ) -> &Option< Cow< 'a, T > >
  {
    self.as_ref()
  }
}

// impl< 'a, T, Marker > AsRef< T > for MaybeAs< 'a, T, Marker >
// where
//   T : Clone,
//   Self : 'a,
// {
//   fn as_ref( &self ) -> &'a T
//   {
//     match &self.0
//     {
//       Some( src ) =>
//       {
//         match src
//         {
//           Cow::Borrowed( src ) => src,
//           Cow::Owned( src ) => &src,
//         }
//       },
//       None => panic!( "MaybeAs is None" ),
//     }
//   }
// }
//
// impl< 'a, T, Marker > Deref for MaybeAs< 'a, T, Marker >
// where
//   T : Clone,
// {
//   type Target = T;
//   fn deref( &self ) -> &'a T
//   {
//     self.as_ref()
//   }
// }

// impl< 'a, T, Marker > From< &T >
// for MaybeAs< 'a, T, Marker >
// where
//   T : std::borrow::ToOwned + ?Sized,
//   < T as std::borrow::ToOwned >::Owned : Clone,
//   // T : Clone,
// {
//   fn from( src : &T ) -> Self
//   {
//     MaybeAs::new( src )
//   }
// }

// impl< 'a, T, Marker > From< < T as std::borrow::ToOwned >::Owned >
// for MaybeAs< 'a, T, Marker >
// where
//   T : std::borrow::ToOwned + ?Sized,
//   < T as std::borrow::ToOwned >::Owned : Clone,
//   // T : Clone,
// {
//   fn from( src : < T as std::borrow::ToOwned >::Owned ) -> Self
//   {
//     MaybeAs::new( src )
//   }
// }

impl< 'a, T, Marker > From< Cow< 'a, T > >
for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  // T : Clone,
{
  fn from( src : Cow< 'a, T > ) -> Self
  {
    MaybeAs::new_with_inner( Some( src ) )
  }
}

impl< 'a, T, Marker > From< Option< Cow< 'a, T > > >
for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  // T : Clone,
{
  fn from( src : Option< Cow< 'a, T > > ) -> Self
  {
    MaybeAs::new_with_inner( src )
  }
}

impl< 'a, T, Marker > From< &'a T >
for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  // T : Clone,
{
  fn from( src : &'a T ) -> Self
  {
    MaybeAs::new_with_ref( src )
  }
}

// impl< 'a, T, Marker > From< () > for MaybeAs< 'a, T, Marker >
// where
//   T : (),
// {
//   fn from( src : &'a T ) -> Self
//   {
//     MaybeAs( None )
//   }
// }

// xxx : more from

// impl< 'a, T, Marker > From< MaybeAs< 'a, T, Marker > > for &'a T
// where
//   T : Clone,
// {
//   fn from( wrapper : MaybeAs< 'a, T, Marker > ) -> &'a T
//   {
//     wrapper.0
//   }
// }

impl< 'a, T, Marker > Default for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  < T as std::borrow::ToOwned >::Owned : Default,
  // T : Clone,
  // T : Default,
{
  fn default() -> Self
  {
    MaybeAs::new( < T as std::borrow::ToOwned >::Owned::default() )
  }
}

impl< 'a, T, Marker > fmt::Debug for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone + fmt::Debug,
  T : fmt::Debug,
  // T : Clone,
{
  fn fmt( &self, f : &mut fmt::Formatter< '_ > ) -> fmt::Result
  {
    f.debug_struct( "MaybeAs" )
    .field( "0", &self.0 )
    .finish()
  }
}

impl< 'a, T, Marker > PartialEq for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  T : PartialEq,
{
  fn eq( &self, other : &Self ) -> bool
  {
    self.as_ref() == other.as_ref()
  }
}

impl< 'a, T, Marker > Eq for MaybeAs< 'a, T, Marker >
where
  T : std::borrow::ToOwned + ?Sized,
  < T as std::borrow::ToOwned >::Owned : Clone,
  T : Eq,
{
}
