// src/storage/disk.rs 
// 磁盘管理器：负责读写磁盘文件 

use super::page::{Page, PageId, PAGE_SIZE}; 
use std::fs::{File, OpenOptions}; 
use std::io::{Read, Seek, SeekFrom, Write}; 
use std::path::PathBuf; 

/// 磁盘管理器 
pub struct DiskManager { 
    file: File, 
    #[allow(dead_code)]
    file_path: PathBuf, 
    reads: u64, 
    writes: u64, 
} 

impl DiskManager { 
    /// 打开或创建数据文件 
    pub fn new(path: &str) -> Result<Self, String> { 
        let file_path = PathBuf::from(path); 
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&file_path)
            .map_err(|e| format!("Failed to open file: {}", e))?; 
        
        Ok(DiskManager { 
            file, 
            file_path, 
            reads: 0, 
            writes: 0, 
        }) 
    } 

    /// 读取指定页 
    pub fn read_page(&mut self, page_id: PageId) -> Result<Page, String> { 
        let offset = (page_id as u64) * PAGE_SIZE as u64; 
        self.file 
            .seek(SeekFrom::Start(offset)) 
            .map_err(|e| format!("Failed to seek: {}", e))?; 
        
        let mut buffer = [0; PAGE_SIZE]; 
        self.file 
            .read_exact(&mut buffer) 
            .map_err(|e| format!("Failed to read page {}: {}", page_id, e))?; 
        
        self.reads += 1; 
        Ok(Page::from_bytes(page_id, &buffer)) 
    } 

    /// 写入指定页 
    pub fn write_page(&mut self, page: &Page) -> Result<(), String> { 
        let offset = (page.page_id as u64) * PAGE_SIZE as u64; 
        self.file 
            .seek(SeekFrom::Start(offset)) 
            .map_err(|e| format!("Failed to seek: {}", e))?; 
        
        self.file 
            .write_all(page.get_data()) 
            .map_err(|e| format!("Failed to write page {}: {}", page.page_id, e))?; 
        
        self.file 
            .flush() 
            .map_err(|e| format!("Failed to flush: {}", e))?; 
        
        self.writes += 1; 
        Ok(()) 
    } 

    /// 获取文件大小（页数） 
    pub fn get_num_pages(&mut self) -> Result<u64, String> { 
        let file_len = self.file 
            .seek(SeekFrom::End(0)) 
            .map_err(|e| format!("Failed to get file size: {}", e))?; 
        Ok(file_len / PAGE_SIZE as u64) 
    } 

    /// 获取读写统计 
    pub fn stats(&self) -> (u64, u64) { 
        (self.reads, self.writes) 
    } 
} 

impl Default for DiskManager { 
    fn default() -> Self { 
        DiskManager::new("data.db").unwrap() 
    } 
} 

#[cfg(test)] 
mod tests { 
    use super::*; 
    use tempfile::NamedTempFile; 

    #[test] 
    fn test_disk_read_write() { 
        let temp_file = NamedTempFile::new().unwrap(); 
        let path = temp_file.path().to_str().unwrap(); 
        
        let mut disk = DiskManager::new(path).unwrap(); 
        let mut page = Page::new(0); 
        page.write(0, b"Hello, Disk!").unwrap(); 
        
        disk.write_page(&page).unwrap(); 
        
        let read_page = disk.read_page(0).unwrap(); 
        assert_eq!(read_page.read(0, 12).unwrap(), b"Hello, Disk!"); 
    } 

    #[test] 
    fn test_disk_num_pages() { 
        let temp_file = NamedTempFile::new().unwrap(); 
        let path = temp_file.path().to_str().unwrap(); 
        
        let mut disk = DiskManager::new(path).unwrap(); 
        assert_eq!(disk.get_num_pages().unwrap(), 0); 
        
        let page = Page::new(0); 
        disk.write_page(&page).unwrap(); 
        
        assert_eq!(disk.get_num_pages().unwrap(), 1); 
    } 
}
