use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use chrono::DateTime;

#[derive(FromRow, Serialize, Deserialize)]
pub struct File {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub uploaded_by: i32,
    pub uploaded_at: DateTime,
    pub project_id: Option<i32>,
    pub permissions: Vec<(User, Permission)>, // A list of user-permission pairs
    pub task_id: Option<i32>,
}

#[derive(Debug, Insertable)]
#[table_name = "file_entries"]
pub struct NewFileEntry {
    pub filename: String,
    pub url: String,
}


pub struct FileComment {
    pub id: u32,
    pub file_id: u32,
    pub author: User,
    pub content: String,
}

#[derive(Debug, Clone)]
pub enum Permission {
    Read,
    Write,
    Delete,
}

impl File {
    pub fn new(id: u32, name: String, url: String, owner: User) -> Self {
        Self {
            id,
            name,
            url,
            owner,
            permissions: vec![],
            version: 1, // Initial version
        }
    }

    pub fn update_file(&mut self, new_url: String) {
        self.url = new_url;
        self.update_version(); // Increment the version
    }

    pub fn share_with(&mut self, user: User, permission: Permission) {
        self.add_permission(user, permission);
    }

    pub fn add_comment(&mut self, comment: FileComment, db: &dyn FileCommentRepository) -> Result<(), DatabaseError> {
        let comment_id = db.create_file_comment(comment)?;
        self.comments.push(FileComment { id: comment_id, ..comment });
        Ok(())
    }

    pub fn add_permission(&mut self, user: User, permission: Permission) {
        self.permissions.push((user, permission));
    }

    pub fn check_permission(&self, user: &User, permission: Permission) -> bool {
        self.permissions.iter().any(|(u, p)| u.id == user.id && *p == permission)
    }

    pub fn update_version(&mut self) {
        self.version += 1;
    }

    pub fn preview(&self) -> String {
        let mime_type = mime_guess::from_path(&self.url).first().unwrap_or("unknown/unknown");
    
        match mime_type.split('/').next().unwrap_or("unknown") {
            "image" => "Image preview unavailable yet".to_owned(),
            "text" => "Text preview unavailable yet".to_owned(),
            _ => format!("No preview available for file type: {}", mime_type),
        }
    }
    


pub struct FileComment {
    pub id: u32,
    pub file_id: u32,
    pub author: User,
    pub content: String,
}

}

#[derive(Debug, Clone)]
pub struct Folder {
    pub id: u32,
    pub name: String,
    pub files: Vec<File>,
}

impl Folder {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            files: vec![],
        }
    }

    pub fn add_file(&mut self, file: File) {
        self.files.push(file);
    }

    pub fn list_files(&self) -> Vec<&File> {
        self.files.iter().collect()
    }
}
