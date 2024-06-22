/// Internal namespace.
pub( crate ) mod private
{

//   pub trait TransitiveTryFrom< IntoTransitive, Transitive, Error > : Sized
//   {
//     // type Error;
//
//     // Required method
//     fn transitive_try_from( value : IntoTransitive ) -> Result< Self, Error >;
//   }
//
//   impl< IntoTransitive, Transitive, Final, Error > TransitiveTryFrom< IntoTransitive, Transitive, Error > for Final
//   where
//     IntoTransitive : TryInto< Transitive >,
//     Error : From< < IntoTransitive as TryInto< Transitive > >::Error >,
//     Final : TryFrom< Transitive, Error = Error >,
//     < Final as TryFrom< Transitive > >::Error : From< < IntoTransitive as TryInto< Transitive > >::Error >
//   {
//     // type Error = Error;
//
//     #[ inline( always ) ]
//     fn transitive_try_from( src : IntoTransitive ) -> Result< Self, Error >
//     {
//       let src2 = TryInto::< Transitive >::try_into( src )?;
//       TryFrom::< Transitive >::try_from( src2 )
//     }
//   }

//   pub trait TransitiveTryFrom< IntoTransitive, Error >
//   {
//
//     fn transitive_try_from< Transitive >( src : IntoTransitive ) -> Result< Self, Error >
//     where
//       IntoTransitive : TryInto< Transitive >,
//       Error : From< < IntoTransitive as TryInto< Transitive > >::Error >,
//       Self : TryFrom< Transitive, Error = Error >,
//       < Self as TryFrom< Transitive > >::Error : From< < IntoTransitive as TryInto< Transitive > >::Error >,
//     {
//       let src2 = TryInto::< Transitive >::try_into( src )?;
//       TryFrom::< Transitive >::try_from( src2 )
//     }
//
//   }
//
//   impl< IntoTransitive, Error, T > TransitiveTryFrom< IntoTransitive, Error > for T
//   {
//   }

  pub trait TransitiveTryFrom< Transitive, IntoTransitive, Error >
  where
    IntoTransitive : TryInto< Transitive >,
    Error : From< < IntoTransitive as TryInto< Transitive > >::Error >,
    Self : TryFrom< Transitive, Error = Error >,
    < Self as TryFrom< Transitive > >::Error : From< < IntoTransitive as TryInto< Transitive > >::Error >,
  {

    fn transitive_try_from( src : IntoTransitive ) -> Result< Self, Error >
    {
      let src2 = TryInto::< Transitive >::try_into( src )?;
      TryFrom::< Transitive >::try_from( src2 )
    }

  }

  // impl< IntoTransitive, Error, T > TransitiveTryFrom< IntoTransitive, Error > for T
  // {
  // }

//   impl< IntoTransitive, T > TransitiveTryFrom< IntoTransitive > for T
//   where
//     IntoTransitive : TryInto< T >,
//     // Error : From< < IntoTransitive as TryInto< T > >::Error >,
//   {
//     type Error = < IntoTransitive as TryInto< T > >::Error;
//
//     #[ inline( always ) ]
//     fn transitive_try_from( src : IntoTransitive ) -> Result< Self, Self::Error >
//     {
//       TryInto::< T >::try_into( src )
//     }
//   }

//   impl< T, U > TransitiveTryInto< U > for T
//   where
//     U : TransitiveTryFrom< T >,
//   {
//     type Error = U::Error;
//
//     #[ inline ]
//     fn transitive_try_into( self ) -> Result< U, U::Error >
//     {
//       U::transitive_try_from( self )
//     }
//   }
//
//   pub trait TransitiveTryInto< T > : Sized
//   {
//     type Error;
//
//     // Required method
//     fn transitive_try_into( self ) -> Result< T, Self::Error >;
//   }

}

crate::mod_interface!
{
  exposed use TransitiveTryFrom;
  // exposed use TransitiveTryInto;
}
