package zone.hwj.vita.tools;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.util.ArrayList;
import java.util.List;
import java.util.concurrent.ConcurrentLinkedQueue;
import zone.hwj.vita.def.NativeBuffer;

public class SlicedBufferPool { // TODO: 目前是模拟逻辑，需提供真实的池化实现

    private final Arena arena;
    private final int blockSize;

    private final ConcurrentLinkedQueue<MemorySegment> pool = new ConcurrentLinkedQueue<>();

    public static SlicedBufferPool common() {
        final class Holder {
            static final SlicedBufferPool INSTANCE = new SlicedBufferPool(4096, 1000);
        }

        return Holder.INSTANCE;
    }

    SlicedBufferPool(int blockSize, int initCap) {
        this.blockSize = blockSize;
        this.arena = Arena.ofShared();
        for (int i = 0; i < initCap; i++) {
            pool.add(arena.allocate(blockSize));
        }
    }

    public int getBlockSize() {
        return blockSize;
    }

    public List<NativeBuffer> acquire(int size) {

        if (size <= blockSize) {
            NativeBuffer single = pollOrCreate(size);
            return List.of(single);
        }

        List<NativeBuffer> buffers = new ArrayList<>(size / blockSize + 1);
        int remaining = size;
        while (remaining > blockSize) {
            NativeBuffer block = pollOrCreate(blockSize);
            remaining -= blockSize;
            buffers.add(block);
        }
        if (remaining > 0) {
            NativeBuffer lastBlock = pollOrCreate(remaining);
            buffers.add(lastBlock);
        }

        return buffers;
    }

    public List<NativeBuffer> acquire(byte[] bytes) {

        int len = bytes.length;
        if (len < blockSize) {
            NativeBuffer single = pollOrCreate(len);
            single.write(0, 0, len, bytes);
            return List.of(single);
        }

        List<NativeBuffer> buffers = new ArrayList<>(len / blockSize + 1);
        int remaining = len;
        int srcOffset = 0;
        while (remaining > blockSize) {
            NativeBuffer block = pollOrCreate(blockSize);
            block.write(srcOffset, 0, blockSize, bytes);
            remaining -= blockSize;
            srcOffset += blockSize;
            buffers.add(block);
        }
        if (remaining > 0) {
            NativeBuffer lastBlock = pollOrCreate(remaining);
            lastBlock.write(srcOffset, 0, remaining, bytes);
            buffers.add(lastBlock);
        }

        return buffers;
    }

    public NativeBuffer resize(NativeBuffer buffer, int newSize) {
        if (newSize > blockSize) {
            throw new IllegalArgumentException("New size should be less than blockSize: " + blockSize);
        }
        MemorySegment newSeg = buffer.getDataPtr().asSlice(0, newSize);
        return NativeBuffer.from(arena, null, newSize, newSeg, false);
    }

    public void release(NativeBuffer buffer) {
        if (buffer == null || buffer.getDataPtr() == null) {
            return;
        }
        pool.offer(buffer.getDataPtr());
    }

    public void close() {
        arena.close();
        pool.clear();
    }

    private NativeBuffer pollOrCreate(int len) {
        MemorySegment segment = pool.poll();
        MemorySegment slice;
        if (segment != null) {
            slice = segment.asSlice(0, len);
        } else {
            slice = arena.allocate(blockSize).asSlice(0, len);
        }
        return NativeBuffer.from(arena, null, len, slice, false);
    }
}
