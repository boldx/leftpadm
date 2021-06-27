#include <linux/bug.h>
#include <linux/printk.h>
#include <linux/uaccess.h>
#include <linux/version.h>
#include <linux/mutex.h>
#include <linux/spinlock.h>


void bug_helper(void)
{
    BUG();
}

int access_ok_helper(const void __user *addr, unsigned long n)
{
#if LINUX_VERSION_CODE >= KERNEL_VERSION(5, 0, 0) /* v5.0-rc1~46 */
    return access_ok(addr, n);
#else
    return access_ok(0, addr, n);
#endif
}

/* see https://github.com/rust-lang/rust-bindgen/issues/1671 */
_Static_assert(__builtin_types_compatible_p(size_t, uintptr_t),
               "size_t must match uintptr_t, what architecture is this??");

void spin_lock_init_wrapper(spinlock_t *lock) { spin_lock_init(lock); }
void spin_lock_wrapper(spinlock_t *lock) { spin_lock(lock); }
void spin_unlock_wrapper(spinlock_t *lock) { spin_unlock(lock); }

void mutex_init_wrapper(struct mutex *lock) { mutex_init(lock); }
void mutex_lock_wrapper(struct mutex *lock) { mutex_lock(lock); }
void mutex_unlock_wrapper(struct mutex *lock) { mutex_unlock(lock); }

