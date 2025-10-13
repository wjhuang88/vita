package zone.hwj.vita.def;

import java.lang.foreign.Arena;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.VarHandle;
import java.util.UUID;
import zone.hwj.vita.NativeTools;

public class NativeBuffer {

    public static final long ID_LEN = 16;

    public static final MemoryLayout LAYOUT = MemoryLayout.structLayout(
            ValueLayout.JAVA_INT.withName("size"),
            ValueLayout.JAVA_BOOLEAN.withName("managed"),
            ValueLayout.JAVA_BOOLEAN.withName("data_managed"),
            MemoryLayout.paddingLayout(ValueLayout.ADDRESS.byteSize() - (ValueLayout.JAVA_BOOLEAN.byteSize() << 1) - ValueLayout.JAVA_INT.byteSize()),
            ValueLayout.ADDRESS.withTargetLayout(ValueLayout.JAVA_BYTE).withName("request_id"),
            ValueLayout.ADDRESS.withTargetLayout(ValueLayout.JAVA_BYTE).withName("data")
    ).withName("JBuffer");

    public static final MemoryLayout ADDRESS_LAYOUT = ValueLayout.ADDRESS.withTargetLayout(LAYOUT);

    private static final VarHandle REQUEST_ID = NativeTools.makeVarHandle(LAYOUT, "request_id");
    private static final VarHandle SIZE = NativeTools.makeVarHandle(LAYOUT, "size");
    private static final VarHandle DATA = NativeTools.makeVarHandle(LAYOUT, "data");
    private static final VarHandle MANAGED = NativeTools.makeVarHandle(LAYOUT, "managed");
    private static final VarHandle DATA_MANAGED = NativeTools.makeVarHandle(LAYOUT, "data_managed");

    private final MemorySegment segment;
    private final MemorySegment dataSegment;
    private byte[] bytes;
    private UUID idCache;

    public NativeBuffer(Arena arena, MemorySegment segment) {
        long segSize = segment.byteSize();
        long layoutSize = LAYOUT.byteSize();
        if (segSize != layoutSize) {
            throw new IllegalArgumentException("segment's byte size does not match layout's. (" + segSize + " vs " + layoutSize + ")");
        }

        boolean external = (boolean) MANAGED.get(segment);
        this.segment = external ? segment.reinterpret(arena, ptr -> {
            try {
                NativeTools.FREE_HANDLE.invokeExact(ptr);
            } catch (Throwable e) {
                throw new IllegalStateException(e);
            }
        }) : segment;

        this.dataSegment = ((MemorySegment) DATA.get(segment)).reinterpret((int) SIZE.get(segment), arena, null);
    }

    public static NativeBuffer from(Arena arena, MemorySegment idSeg, int size, MemorySegment dataSeg, boolean dataExternal) {
        MemorySegment bufferSeg = arena.allocate(LAYOUT);
        if (idSeg != null) {
            REQUEST_ID.set(bufferSeg, idSeg);
        }
        SIZE.set(bufferSeg, size);
        DATA.set(bufferSeg, dataSeg);
        MANAGED.set(bufferSeg, false);
        if (dataExternal) {
            DATA_MANAGED.set(bufferSeg, true);
        } else {
            DATA_MANAGED.set(bufferSeg, false);
        }
        return new NativeBuffer(arena, bufferSeg);
    }

    public int getSize() {
        return (int) SIZE.get(segment);
    }

    public UUID getRequestId() {
        if (idCache == null) {
            idCache = NativeTools.makeUUID(getRequestIdPtr());
        }
        return idCache;
    }

    public MemorySegment getRequestIdPtr() {
        return ((MemorySegment) REQUEST_ID.get(segment)).reinterpret(ID_LEN);
    }
    public MemorySegment getDataPtr() {
        return this.dataSegment;
    }

    public MemorySegment getPtr() {
        return segment;
    }

    public byte read(int index) {
        int size = getSize();
        if (index < 0 || index > size) {
            throw new IndexOutOfBoundsException("NativeBuffer has size: " + size + ", index: " + index + "out of bound");
        }
        MemorySegment dataPtr = getDataPtr();
        return dataPtr.get(ValueLayout.JAVA_BYTE, index);
    }

    public byte[] copyContent() {
        if (bytes == null) {
            int size = getSize();
            byte[] handled = new byte[size];
            MemorySegment data = getDataPtr();
            MemorySegment.copy(data, ValueLayout.JAVA_BYTE, 0, handled, 0, size);
            bytes = handled;
        }
        return bytes;
    }
}
