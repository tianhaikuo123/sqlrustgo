// src/storage/buffer.rs 
// 缓冲区管理器：缓存经常访问的页 

use super::page::{Page, PageId}; 
use std::collections::HashMap; 
use std::collections::VecDeque; 

/// 缓冲区管理器 
pub struct BufferPool { 
    pages: HashMap<PageId, Page>,     // 页缓存 
    capacity: usize,                   // 最大缓存页数 
    lru: VecDeque<PageId>,             // LRU 队列 
    hits: u64,                         // 缓存命中次数 
    misses: u64,                       // 缓存未命中次数 
} 

impl BufferPool { 
    /// 创建新的缓冲区管理器 
    pub fn new(capacity: usize) -> Self { 
        BufferPool { 
            pages: HashMap::new(), 
            capacity, 
            lru: VecDeque::new(), 
            hits: 0, 
            misses: 0, 
        } 
    } 

    /// 获取页（如果不在缓存中，返回 None） 
    pub fn get_page(&mut self, page_id: PageId) -> Option<&mut Page> { 
        if self.pages.contains_key(&page_id) { 
            self.hits += 1; 
            self.update_lru(page_id); 
            self.pages.get_mut(&page_id) 
        } else { 
            self.misses += 1; 
            None 
        } 
    } 

    /// 检查页是否在缓存中 
    pub fn has_page(&self, page_id: PageId) -> bool { 
        self.pages.contains_key(&page_id) 
    } 

    /// 添加页到缓存 
    pub fn add_page(&mut self, page: Page) -> Result<(), String> { 
        let page_id = page.page_id; 
        
        // 如果已存在，直接返回 
        if self.pages.contains_key(&page_id) { 
            return Ok(()); 
        } 
        
        // 如果缓存已满，淘汰一个页 
        if self.pages.len() >= self.capacity { 
            self.evict()?; 
        } 
        
        self.pages.insert(page_id, page); 
        self.update_lru(page_id); 
        Ok(()) 
    } 

    /// 更新 LRU 队列 
    fn update_lru(&mut self, page_id: PageId) { 
        self.lru.retain(|&id| id != page_id); 
        self.lru.push_back(page_id); 
    } 

    /// 淘汰最久未使用的页 
    fn evict(&mut self) -> Result<(), String> { 
        while let Some(page_id) = self.lru.pop_front() { 
            if let Some(page) = self.pages.get(&page_id) { 
                if page.pin_count > 0 { 
                    self.update_lru(page_id); 
                    continue; 
                } 
                self.pages.remove(&page_id); 
                return Ok(()); 
            } 
        } 
        Err("No evictable page found".to_string()) 
    } 

    /// 强制将所有脏页写回磁盘 
    pub fn flush_all(&mut self) -> Result<(), String> { 
        for page in self.pages.values_mut() { 
            if page.is_dirty { 
                page.is_dirty = false; 
            } 
        } 
        Ok(()) 
    } 

    /// 获取缓存命中率 
    pub fn hit_rate(&self) -> f64 { 
        let total = self.hits + self.misses; 
        if total == 0 { 
            0.0 
        } else { 
            self.hits as f64 / total as f64 
        } 
    } 

    /// 获取缓存统计信息 
    pub fn stats(&self) -> (usize, u64, u64, f64) { 
        (self.pages.len(), self.hits, self.misses, self.hit_rate()) 
    }

    /// 获取所有缓存页的 ID 
    pub fn page_ids(&self) -> Vec<PageId> {
        self.pages.keys().copied().collect()
    } 
} 

#[cfg(test)] 
mod tests { 
    use super::*; 
    use super::super::page::Page; 

    #[test] 
    fn test_buffer_pool_add() { 
        let mut pool = BufferPool::new(3); 
        let page = Page::new(1); 
        
        pool.add_page(page).unwrap(); 
        assert!(pool.has_page(1)); 
    } 

    #[test] 
    fn test_buffer_pool_get() { 
        let mut pool = BufferPool::new(3); 
        let page = Page::new(1); 
        pool.add_page(page).unwrap(); 
        
        let result = pool.get_page(1); 
        assert!(result.is_some()); 
    } 

    #[test] 
    fn test_buffer_pool_eviction() { 
        let mut pool = BufferPool::new(2); 
        
        pool.add_page(Page::new(1)).unwrap(); 
        pool.add_page(Page::new(2)).unwrap(); 
        pool.add_page(Page::new(3)).unwrap(); 
        
        assert!(pool.has_page(3)); 
        assert_eq!(pool.pages.len(), 2); 
    } 
}
