package zone.hwj.vita.def;

import java.lang.foreign.Arena;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.nio.charset.StandardCharsets;

public class PathHolder {

    private final MemorySegment segment;

    private PathHolder(MemorySegment segment) {
        this.segment = segment;
    }

    public static PathHolder of(Arena arena, String pattern) {
        byte[] resultBytes = pattern.getBytes(StandardCharsets.UTF_8);
        MemorySegment resultSeg = arena.allocateFrom(ValueLayout.JAVA_BYTE, resultBytes);
        MemorySegment pathSeg = NativeBuffer.from(arena, null, resultBytes.length, resultSeg, false).getPtr();
        return new PathHolder(pathSeg);
    }

    public MemorySegment getPtr() {
        return segment;
    }
}
