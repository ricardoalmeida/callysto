use crate::definitions::{TaskDef, ServiceDef};
use crate::table::CTable;
use crate::kafka::CTopic;
use lever::sync::atomics::AtomicBox;
use std::sync::Arc;
use lever::prelude::{LOTable, HOPTable};
use std::sync::atomic::{AtomicUsize, Ordering};
use crate::prelude::CronJob;

pub struct Callysto<Store>
where
    Store: 'static
{
    app_name: String,
    storage: Store,
    brokers: String,
    stubs: Arc<AtomicUsize>,
    tasks: LOTable<usize, Arc<dyn TaskDef<Store>>>,
    timers: LOTable<usize, Arc<dyn TaskDef<Store>>>,
    cronjobs: LOTable<usize, Arc<CronJob<Store>>>,
    services: LOTable<usize, Arc<dyn ServiceDef<Store>>>,
}

impl Callysto<()> {
    #[must_use]
    pub fn new() -> Self {
        Self::with_storage(())
    }
}

impl Default for Callysto<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Store> Callysto<Store>
where
    Store: Clone + Send + Sync + 'static
{
    pub fn with_storage(storage: Store) -> Self {
        Self {
            app_name: "callysto-app".to_owned(),
            storage,
            stubs: Arc::new(AtomicUsize::default()),
            brokers: "localhost:9092".to_owned(),
            tasks: LOTable::default(),
            timers: LOTable::default(),
            cronjobs: LOTable::default(),
            services: LOTable::default()
        }
    }

    pub fn with_name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.app_name = name.as_ref().to_string();
        self
    }

    pub fn with_brokers<T: AsRef<str>>(&mut self, brokers: T) -> &mut Self {
        self.brokers = brokers.as_ref().to_string();
        self
    }

    pub fn task(&self, t: impl TaskDef<Store>) -> &Self {
        let stub = self.stubs.fetch_add(1, Ordering::AcqRel);
        self.tasks.insert(stub, Arc::new(t));
        self
    }

    pub fn timer(&self, t: impl TaskDef<Store>) -> &Self {
        let stub = self.stubs.fetch_add(1, Ordering::AcqRel);
        self.timers.insert(stub, Arc::new(t));
        self
    }

    pub fn service(&self, s: impl ServiceDef<Store>) -> &Self {
        let stub = self.stubs.fetch_add(1, Ordering::AcqRel);
        self.services.insert(stub, Arc::new(s));
        self
    }

    pub fn crontab<C: AsRef<str>>(&self, cron_expr: C, t: impl TaskDef<Store>) -> &Self {
        let stub = self.stubs.fetch_add(1, Ordering::AcqRel);
        let cron_job = Arc::new(CronJob::new(cron_expr, t));
        self.cronjobs.insert(stub, cron_job);
        self
    }

    pub fn topic(&self) -> CTopic {
        todo!()
    }

    pub fn table(&self) -> CTable {
        todo!()
    }
}