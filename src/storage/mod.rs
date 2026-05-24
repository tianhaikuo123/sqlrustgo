// src/storage/mod.rs 
// 存储引擎模块入口 

pub mod page; 
pub mod buffer; 
pub mod disk; 

pub use page::{Page, PageId, PAGE_SIZE}; 
pub use buffer::BufferPool; 
pub use disk::DiskManager; 

/// 存储引擎（组合了缓冲区和磁盘） 
pub struct StorageEngine { 
    pub buffer_pool: BufferPool, 
    pub disk: DiskManager, 
} 

impl StorageEngine { 
    /// 创建新的存储引擎 
    pub fn new(buffer_capacity: usize, db_path: &str) -> Result<Self, String> { 
        Ok(StorageEngine { 
            buffer_pool: BufferPool::new(buffer_capacity), 
            disk: DiskManager::new(db_path)?, 
        }) 
    } 

    /// 读取页（优先从缓冲区读） 
    pub fn read_page(&mut self, page_id: PageId) -> Result<&mut Page, String> { 
        // 如果缓冲区已有，直接返回 
        if let Some(page) = self.buffer_pool.get_page(page_id) { 
            return Ok(page); 
        } 
        
        // 从磁盘读取 
        let mut page = self.disk.read_page(page_id)?; 
        page.pin(); 
        self.buffer_pool.add_page(page)?; 
        
        self.buffer_pool 
            .get_page(page_id) 
            .ok_or_else(|| "Failed to load page".to_string()) 
    } 

    /// 写入页（标记为脏） 
    pub fn write_page(&mut self, page_id: PageId, offset: usize, data: &[u8]) -> Result<(), String> { 
        if let Some(page) = self.buffer_pool.get_page(page_id) { 
            page.write(offset, data)?; 
        } else { 
            let mut page = Page::new(page_id); 
            page.write(offset, data)?; 
            self.buffer_pool.add_page(page)?; 
        } 
        Ok(()) 
    } 

    /// 刷新所有脏页到磁盘 
    pub fn flush(&mut self) -> Result<(), String> { 
        // 将所有脏页写回磁盘 
        for page_id in self.buffer_pool.pages.keys() { 
            if let Some(page) = self.buffer_pool.get_page(*page_id) { 
                if page.is_dirty { 
                    self.disk.write_page(page)?; 
                } 
            } 
        } 
        self.buffer_pool.flush_all() 
    } 

    /// 获取统计信息 
    pub fn stats(&self) -> (usize, u64, u64, f64, u64, u64) { 
        let (cache_size, hits, misses, hit_rate) = self.buffer_pool.stats(); 
        let (reads, writes) = self.disk.stats(); 
        (cache_size, hits, misses, hit_rate, reads, writes) 
    } 
} 

#[cfg(test)] 
mod tests { 
    use super::*; 
    use tempfile::NamedTempFile; 

    #[test] 
    fn test_storage_engine_read_write() { 
        let temp_file = NamedTempFile::new().unwrap(); 
        let path = temp_file.path().to_str().unwrap(); 
        
        let mut engine = StorageEngine::new(10, path).unwrap(); 
        
        engine.write_page(0, 0, b"Test data").unwrap(); 
        engine.flush().unwrap(); 
        
        let page = engine.read_page(0).unwrap(); 
        assert_eq!(page.read(0, 10).unwrap(), b"Test data"); 
    } 

    #[test] 
    fn test_storage_engine_stats() { 
        let temp_file = NamedTempFile::new().unwrap(); 
        let path = temp_file.path().to_str().unwrap(); 
        
        let mut engine = StorageEngine::new(10, path).unwrap(); 
        
        engine.write_page(0, 0, b"Data1").unwrap(); 
        engine.write_page(1, 0, b"Data2").unwrap(); 
        
        let stats = engine.stats(); 
        assert_eq!(stats.0, 2); // 2 pages in cache 
    } 
}
