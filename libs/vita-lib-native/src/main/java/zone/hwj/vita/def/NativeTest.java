package zone.hwj.vita.def;

import java.lang.foreign.MemoryLayout;
import java.lang.foreign.MemoryLayout.PathElement;
import java.lang.foreign.MemorySegment;
import java.lang.foreign.ValueLayout;
import java.lang.invoke.MethodHandles;
import java.lang.invoke.VarHandle;

public class NativeTest {
    public static final MemoryLayout LAYOUT = MemoryLayout.structLayout(
            ValueLayout.JAVA_INT.withName("a"),
            MemoryLayout.paddingLayout(4),
            ValueLayout.JAVA_LONG.withName("b"),
            ValueLayout.JAVA_FLOAT.withName("c"),
            MemoryLayout.paddingLayout(4),
            ValueLayout.JAVA_DOUBLE.withName("d")
    ).withName("JTestStruct");

    private static final VarHandle aHandle = LAYOUT.varHandle(PathElement.groupElement("a"));
    private static final VarHandle bHandle = LAYOUT.varHandle(PathElement.groupElement("b"));
    private static final VarHandle cHandle = LAYOUT.varHandle(PathElement.groupElement("c"));
    private static final VarHandle dHandle = LAYOUT.varHandle(PathElement.groupElement("d"));

    private static final VarHandle A;
    private static final VarHandle B;
    private static final VarHandle C;
    private static final VarHandle D;

    static {
        A = MethodHandles.insertCoordinates(aHandle, 1, 0L);
        B = MethodHandles.insertCoordinates(bHandle, 1, 0L);
        C = MethodHandles.insertCoordinates(cHandle, 1, 0L);
        D = MethodHandles.insertCoordinates(dHandle, 1, 0L);
    }

    private final int a;
    private final long b;
    private final float c;
    private final double d;

    public NativeTest(MemorySegment segment) {
        long segSize = segment.byteSize();
        long layoutSize = LAYOUT.byteSize();
        if (segSize != layoutSize) {
            throw new IllegalArgumentException("segment's byte size does not match layout's. (" + segSize + " vs " + layoutSize + ")");
        }
        this.a = (int) A.get(segment);
        this.b = (long) B.get(segment);
        this.c = (float) C.get(segment);
        this.d = (double) D.get(segment);
    }

    public int getA() {
        return a;
    }

    public long getB() {
        return b;
    }

    public float getC() {
        return c;
    }

    public double getD() {
        return d;
    }
}
