use crate::traits::Model;
use sqlx::{Pool, Postgres};

macro_rules! cache {
    ($($name:ident, $plural:ident, $type:ident, $store:ident)*) => {
        pub struct Cache {
            $(
                $plural: <crate::structs::$type as Model>::Map,
            )*
            $(
                $store: crate::structs::LockStore<<crate::structs::$type as Model>::Key>,
            )*
        }

        impl Cache {
            pub async fn hydrate(pool: &Pool<Postgres>) -> Self {
                Self {
                    $(
                        $plural: super::$type::select_all_as_map(pool).await,
                    )*
                    $(
                        $store: crate::structs::LockStore::new(),
                    )*
                }
            }

            paste::paste! {
                $(
                    #[allow(dead_code)]
                    #[inline(always)]
                    pub fn [<get_ $name>](&self, key: &<crate::structs::$type as Model>::Key) -> Option<crate::structs::$type> {
                        self.$plural.get(key).map(|v| v.clone())
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub fn [<insert_ $name>](&self, key: <crate::structs::$type as Model>::Key, value: crate::structs::$type) {
                        self.$plural.insert(key, value);
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub fn [<remove_ $name>](&self, key: &<crate::structs::$type as Model>::Key) {
                        self.$plural.remove(key);
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub fn [<update_ $name>](&self, key: &<crate::structs::$type as Model>::Key, value: &super::$type) {
                        if let Some(mut v) = self.$plural.get_mut(key) {
                            v.clone_from(&value);
                        }
                    }

                    #[allow(dead_code)]
                    #[inline(always)]
                    pub async fn [<lock_ $name>](&self, key: &<crate::structs::$type as Model>::Key) -> crate::structs::LockGuard<<crate::structs::$type as Model>::Key> {
                        self.$store.lock(*key).await
                    }
                )*
            }
        }
    };
}

cache!(
    alliance, alliances, Alliance, alliance_locks
    city, cities, City, city_locks
    nation, nations, Nation, nation_locks
    user, users, User, user_locks
);
