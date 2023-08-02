use cacache;
use futures::executor::block_on;
use ssri::Integrity;

use Static::{Events, LOCAL_DB};


///# 磁盘
pub trait Disk {
	const ERROR_INVALID: &'static str;
	async fn file(&self) -> String {
		LOCAL_DB.as_path().to_str().unwrap().to_string()
	}
	async fn write(&self) -> Integrity;
	async fn read(&self) -> Vec<u8>;
	async fn remove(&self);
	async fn hash_write(&self) -> Integrity;
	async fn hash_read(&self, ees: &Integrity) -> Vec<u8>;
	async fn remove_hash(&self, ees: &Integrity);
}

///# 存储类型
pub struct KVStore<RT: ToString, RG: ToString> {
	pub hash: Option<RT>,
	pub key: Option<RT>,
	pub value: RG,
}

impl From<String> for KVStore<String, String> {
	fn from(value: String) -> Self {
		KVStore { hash: None, key: None, value }
	}
}

impl From<(String, String)> for KVStore<String, String> {
	fn from(value: (String, String)) -> Self {
		KVStore { hash: None, key: Some(value.0), value: value.1 }
	}
}

impl<RT: ToString, RG: ToString> KVStore<RT, RG> {
	///# 字符转换
	pub fn string_with(e: Vec<u8>) -> String {
		String::from_utf8(e).expect(Self::ERROR_INVALID)
	}
	pub fn write_into(&self) -> Events<Integrity> {
		Ok(if let Some(ref e) = self.hash {
			cacache::write_sync(e.to_string(), self.key.as_ref().unwrap().to_string(), self.value.to_string())?
		} else {
			cacache::write_sync(block_on(self.file()), self.key.as_ref().unwrap().to_string(), self.value.to_string())?
		})
	}
	pub fn read_into(&self) -> Events<Vec<u8>> {
		Ok(if let Some(ref e) = self.hash {
			cacache::read_sync(e.to_string(), self.key.as_ref().unwrap().to_string())?
		} else {
			cacache::read_sync(block_on(self.file()), self.key.as_ref().unwrap().to_string())?
		})
	}
	pub fn remove_into(&self) -> Events<()> {
		Ok(if let Some(ref e) = self.hash {
			cacache::remove_sync(e.to_string(), self.key.as_ref().unwrap().to_string())?
		} else {
			cacache::remove_sync(block_on(self.file()), self.key.as_ref().unwrap().to_string())?
		})
	}
	///# 无锁哈希
	pub fn write_to_hash_algo(&self) -> Events<Integrity> {
		Ok(if let Some(ref e) = self.hash {
			cacache::write_hash_sync_with_algo(cacache::Algorithm::Sha512, e.to_string(), self.value.to_string())?
		} else {
			cacache::write_hash_sync_with_algo(cacache::Algorithm::Sha512, block_on(self.file()), self.value.to_string())?
		})
	}
	///# 无锁
	pub fn write_to_algo(&self) -> Events<Integrity> {
		Ok(if let Some(ref e) = self.hash {
			cacache::write_sync_with_algo(cacache::Algorithm::Sha512, e.to_string(), self.key.as_ref().unwrap().to_string(), self.value.to_string())?
		} else {
			cacache::write_sync_with_algo(cacache::Algorithm::Sha512, block_on(self.file()), self.key.as_ref().unwrap().to_string(), self.value.to_string())?
		})
	}
}

impl<RT: ToString, RG: ToString> Disk for KVStore<RT, RG> {
	const ERROR_INVALID: &'static str = "ASYNC CACHE ERROR";
	async fn write(&self) -> Integrity {
		if let Some(ref e) = self.hash {
			cacache::write(e.to_string(), self.key.as_ref().unwrap().to_string(), self.value.to_string()).await.expect(Self::ERROR_INVALID)
		} else {
			cacache::write(self.file().await, self.key.as_ref().unwrap().to_string(), self.value.to_string()).await.expect(Self::ERROR_INVALID)
		}
	}
	async fn read(&self) -> Vec<u8> {
		if let Some(ref e) = self.hash {
			cacache::read(e.to_string(), self.key.as_ref().unwrap().to_string()).await.expect(Self::ERROR_INVALID)
		} else {
			cacache::read(self.file().await, self.key.as_ref().unwrap().to_string()).await.expect(Self::ERROR_INVALID)
		}
	}
	async fn remove(&self) {
		if let Some(ref e) = self.hash {
			cacache::remove(e.to_string(), self.key.as_ref().unwrap().to_string()).await.expect(Self::ERROR_INVALID);
		} else {
			cacache::remove(self.file().await, self.key.as_ref().unwrap().to_string()).await.expect(Self::ERROR_INVALID);
		}
	}
	async fn hash_write(&self) -> Integrity {
		if let Some(ref e) = self.hash {
			cacache::write_hash(e.to_string(), self.value.to_string()).await.expect(Self::ERROR_INVALID)
		} else {
			cacache::write_hash(self.file().await, self.value.to_string()).await.expect(Self::ERROR_INVALID)
		}
	}
	async fn hash_read(&self, ees: &Integrity) -> Vec<u8> {
		if let Some(ref e) = self.hash {
			cacache::read_hash(e.to_string(), ees).await.expect(Self::ERROR_INVALID)
		} else {
			cacache::read_hash(self.file().await, ees).await.expect(Self::ERROR_INVALID)
		}
	}
	async fn remove_hash(&self, ees: &Integrity) {
		if let Some(ref e) = self.hash {
			cacache::remove_hash(e.to_string(), ees).await.expect(Self::ERROR_INVALID)
		} else {
			cacache::remove_hash(self.file().await, ees).await.expect(Self::ERROR_INVALID)
		}
	}
}