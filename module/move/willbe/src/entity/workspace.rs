mod private
{
  use crate::*;

  // qqq : for Bohdan : bad
  // use std::*;

  use std::slice;
  use former::Former;

  /// Stores information about the current workspace.
  #[ derive( Debug, Clone ) ]
  pub struct Workspace
  {
    /// Metadata of the workspace, containing detailed information about the packages, dependencies, and other workspace-related data.
    pub metadata : cargo_metadata::Metadata,
    /// The directory containing the manifest file (`Cargo.toml`) of the workspace.
    pub crate_dir : CrateDir,
  }

  /// Represents errors related to workspace operations.
  #[ derive( Debug, error::typed::Error ) ]
  pub enum WorkspaceInitError
  {
    /// Something went wrong with path to a workspace.
    #[ error( "Path error. Details: {0}" ) ]
    Path( #[ from ] PathError ),
    /// Something went wrong with the workspace' data
    #[ error( "Can not load workspace data. Details: {0}" ) ]
    Metadata( #[ from ] cargo_metadata::Error ),
    /// Files error
    #[ error( "I/O error: {0}" ) ]
    IO( #[ from ] std::io::Error ),
  }

  impl TryFrom< CrateDir > for Workspace
  {
    type Error = WorkspaceInitError;

    /// Load data from current directory
    fn try_from( crate_dir : CrateDir ) -> Result< Self, Self::Error >
    {
      let metadata = cargo_metadata::MetadataCommand::new()
      .current_dir( crate_dir.as_ref() )
      .no_deps()
      .exec()?;
      Ok( Self
      {
        metadata,
        crate_dir,
      })
    }

  }

  impl TryFrom< CurrentPath > for Workspace
  {
    type Error = WorkspaceInitError;

    /// Load data from current directory
    fn try_from( cd : CurrentPath ) -> Result< Self, Self::Error >
    {
      Self::try_from( CrateDir::transitive_try_from::< AbsolutePath >( cd )? )
    }

  }

  impl From< cargo_metadata::Metadata > for Workspace
  {
    fn from( metadata : cargo_metadata::Metadata ) -> Self
    {
      // SAFE: `workspace_root` is a path to a`Cargo.toml` file, therefor the parent is the directory
      let path = metadata.workspace_root.as_std_path().parent().unwrap().to_path_buf();
      let path = AbsolutePath::try_from( path ).unwrap();
      Self
      {
        metadata,
        crate_dir : CrateDir::try_from( path ).unwrap(),
      }
    }
  }

  impl Workspace
  {

    /// Returns list of all packages
    pub fn packages< 'a >( &'a self )
    -> core::iter::Map
    <
      slice::Iter< 'a, cargo_metadata::Package >,
      impl Fn( &'a cargo_metadata::Package ) -> WorkspacePackageRef< 'a > + Clone,
    >
    {
      self.metadata.packages.iter().map( WorkspacePackageRef::from )
    }

    /// Returns the path to workspace root
    pub fn workspace_root( &self ) -> CrateDir
    {
      // Safe because workspace_root.as_std_path() is always a path to a directory
      CrateDir::try_from( self.metadata.workspace_root.as_std_path() ).unwrap()
    }

    /// Returns the path to target directory
    pub fn target_directory( &self ) -> &std::path::Path
    {
      self.metadata.target_directory.as_std_path()
    }

    /// Find a package by its manifest file path
    pub fn package_find_by_manifest< 'a, P >( &'a self, manifest_file : P ) -> Option< WorkspacePackageRef< 'a > >
    where
      P : AsRef< std::path::Path >,
    {
      self
      .packages()
      .find( | &p | p.manifest_file().unwrap().as_ref() == manifest_file.as_ref() )
    }

    /// Filter of packages.
    pub fn packages_which< 'a >( &'a self ) -> PackagesFilterFormer< 'a >
    {
      PackagesFilter::former().workspace( self )
    }

  }

  #[ derive( Debug, Former ) ]
  pub struct PackagesFilter< 'a >
  {
    workspace : &'a Workspace,
    crate_dir : Option< CrateDir >,
    manifest_file : Option< ManifestFile >,
  }

  impl< 'a > PackagesFilterFormer< 'a >
  {
    #[ inline( always ) ]
    pub fn filter( self ) -> impl Iterator< Item = WorkspacePackageRef< 'a > > + Clone
    {
      // let filter_crate_dir = if Some( crate_dir ) = self.crate_dir
      // {
      //   | p | p.manifest_file().unwrap().as_ref() == manifest_file.as_ref()
      // }
      std::iter::empty()
    }
  }

  impl Entries for Workspace
  {
    fn entries( &self ) -> impl Iterator< Item = SourceFile > + Clone
    {
      self
      .packages()
      .flat_map( | package | package.entries().collect::< Vec< _ > >() )
      .into_iter()
    }
  }

  impl Sources for Workspace
  {
    fn sources( &self ) -> impl Iterator< Item = SourceFile > + Clone
    {
      self
      .packages()
      .flat_map( | package | package.sources().collect::< Vec< _ > >() )
      .into_iter()
    }
  }

}

//

crate::mod_interface!
{
  exposed use WorkspaceInitError;
  exposed use Workspace;
}
