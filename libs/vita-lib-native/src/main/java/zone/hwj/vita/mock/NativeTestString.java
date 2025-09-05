package zone.hwj.vita.mock;

import java.lang.foreign.Arena;
import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemoryLayout.PathElement;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;
import java.util.function.BiConsumer;

public class NativeTestString {
    public static final MemoryLayout LAYOUT = MemoryLayout.structLayout(
            ValueLayout.JAVA_INT.withName("size"),
            MemoryLayout.paddingLayout(4),
            ValueLayout.ADDRESS.withTargetLayout(ValueLayout.JAVA_BYTE).withName("data")
    ).withName("JString");

    private static final VarHandle sizeHandle = LAYOUT.varHandle(PathElement.groupElement("size"));
    private static final VarHandle dataHandle = LAYOUT.varHandle(PathElement.groupElement("data"));

    private static final VarHandle SIZE;
    private static final VarHandle DATA;

    static {
        SIZE = MethodHandles.insertCoordinates(sizeHandle, 1, 0L);
        DATA = MethodHandles.insertCoordinates(dataHandle, 1, 0L);
    }

    private final MemorySegment segment;
    private final BiConsumer<MemorySegment, Integer> cleanup;
    private byte[] bytes;

    public NativeTestString(MemorySegment segment, BiConsumer<MemorySegment, Integer> cleanup) {
        long segSize = segment.byteSize();
        long layoutSize = LAYOUT.byteSize();
        if (segSize != layoutSize) {
            throw new IllegalArgumentException("segment's byte size does not match layout's. (" + segSize + " vs " + layoutSize + ")");
        }
        this.segment = segment;
        this.cleanup = cleanup;
    }

    private int getSize() {
        return (int) SIZE.get(segment);
    }

    private MemorySegment getData() {
        return (MemorySegment) DATA.get(segment);
    }

    public byte[] getContent(Arena arena) {
        if (bytes == null) {
            int size = getSize();
            byte[] handled = new byte[size];
            MemorySegment data = getData().reinterpret(size, arena, seg -> cleanup.accept(seg, size));
            MemorySegment.copy(data, ValueLayout.JAVA_BYTE, 0, handled, 0, size);
            bytes = handled;
        }
        return bytes;
    }
}
