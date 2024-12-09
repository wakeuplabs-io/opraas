use crate::domain::TProjectVersionControl;
use git2::{IndexAddOption, Repository};

pub struct GitVersionControl;

impl GitVersionControl {
    pub fn new() -> Self {
        Self {}
    }
}

impl TProjectVersionControl for GitVersionControl {
    fn init(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        Repository::init(filepath)?;

        Ok(())
    }

    fn stage(&self, filepath: &str) -> Result<(), Box<dyn std::error::Error>> {
        let repo = Repository::open(filepath)?;

        let mut index = repo.index()?;
        index.add_all(&["."], IndexAddOption::DEFAULT, None)?;
        index.write()?;

        Ok(())
    }

    fn commit(&self, filepath: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        // let repo = Repository::open(filepath)?;
        // let mut index = repo.index()?;

        // let new_tree_oid = index.write_tree()?;
        // let new_tree = repo.find_tree(new_tree_oid)?;

        // // either use the configured author signature
        // let author = repo.signature()?;

        // let head = repo.head()?;
        // let parent = repo.find_commit(head.target().unwrap())?;
        // repo.commit(
        //     Some("HEAD"),
        //     &author,
        //     &author,
        //     message,
        //     &new_tree,
        //     &[&parent],
        // )?;

        let repo = Repository::open(filepath)?;
        let mut index = repo.index()?;

        // Stage all changes
        let new_tree_oid = index.write_tree()?;
        let new_tree = repo.find_tree(new_tree_oid)?;

        // Author signature (you can customize this if needed)
        let author = repo.signature()?;

        // Check if HEAD exists and points to a valid branch
        let head = match repo.head() {
            Ok(head) => Some(head),
            Err(e) if e.code() == git2::ErrorCode::NotFound => None, // No HEAD (e.g., new repo)
            Err(e) => return Err(Box::new(e)),                       // Other errors
        };

        let parents = if let Some(head) = head {
            // Get the current commit pointed by HEAD
            let parent_commit = repo.find_commit(head.target().unwrap())?;
            vec![parent_commit]
        } else {
            // No parent commits (first commit)
            vec![]
        };

        let parent_refs: Vec<&git2::Commit> = parents.iter().collect();

        // Create a new commit
        repo.commit(
            Some("HEAD"), // Update the current branch
            &author,      // Commit author
            &author,      // Commit committer
            message,      // Commit message
            &new_tree,    // The tree object for the commit
            &parent_refs, // Parents of the commit
        )?;

        Ok(())
    }
}
