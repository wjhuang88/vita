package zone.hwj.vita.tools;

import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.Test;
import zone.hwj.vita.def.NativeBuffer;

import java.nio.charset.StandardCharsets;
import java.util.List;
import java.util.concurrent.CountDownLatch;
import java.util.concurrent.atomic.AtomicInteger;

import static org.junit.jupiter.api.Assertions.*;

/**
 * Unit tests for {@link SlicedBufferPool}.
 */
public class SlicedBufferPoolTest {

    private SlicedBufferPool pool;

    @BeforeEach
    void setUp() {
        // 使用较小的配置方便测试
        pool = new SlicedBufferPool(8, 10);
    }

    @Test
    void testAcquireSmallBuffer() {
        byte[] data = "Hi".getBytes(StandardCharsets.UTF_8);
        List<NativeBuffer> buffers = pool.acquire(data);
        assertEquals(1, buffers.size(), "Small data should return one buffer");
        assertNotNull(buffers.getFirst());
    }

    @Test
    void testAcquireLargeBufferSplitting() {
        byte[] data = "HelloWorld!".getBytes(StandardCharsets.UTF_8); // 11 bytes, blockSize = 8
        List<NativeBuffer> buffers = pool.acquire(data);
        assertTrue(buffers.size() > 1, "Should split into multiple blocks");
    }

    @Test
    void testReleaseBufferDoesNotThrow() {
        byte[] data = "12345678".getBytes(StandardCharsets.UTF_8);
        List<NativeBuffer> buffers = pool.acquire(data);
        assertDoesNotThrow(() -> pool.release(buffers.getFirst()));
    }

    @Test
    void testCommonSingleton() {
        SlicedBufferPool a = SlicedBufferPool.common();
        SlicedBufferPool b = SlicedBufferPool.common();
        assertSame(a, b, "Common instance should be singleton");
    }

    @Test
    void testConcurrentAcquireAndRelease() throws InterruptedException {
        int threads = 10;
        CountDownLatch latch = new CountDownLatch(threads);
        AtomicInteger errors = new AtomicInteger(0);

        Runnable worker = () -> {
            try {
                byte[] data = ("DATA-" + Thread.currentThread().threadId()).getBytes(StandardCharsets.UTF_8);
                List<NativeBuffer> bufs = pool.acquire(data);
                bufs.forEach(pool::release);
            } catch (Throwable t) {
                t.printStackTrace();
                errors.incrementAndGet();
            } finally {
                latch.countDown();
            }
        };

        for (int i = 0; i < threads; i++) {
            new Thread(worker).start();
        }

        latch.await();
        assertEquals(0, errors.get(), "All threads should complete successfully");
    }
}