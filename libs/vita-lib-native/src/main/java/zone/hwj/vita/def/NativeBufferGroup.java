package zone.hwj.vita.def;

import java.util.ArrayList;
import java.util.List;
import zone.hwj.vita.tools.pair.Pair;

public class NativeBufferGroup {
    private final List<Node> buffers = new ArrayList<>();
    private int lastEndOffset = 0;

    private NativeBufferGroup(NativeBuffer... init) {
        for (NativeBuffer buffer : init) {
            appendBuffer(buffer);
        }
    }

    public static NativeBufferGroup of(NativeBuffer... buffers) {
        return new NativeBufferGroup(buffers);
    }

    public void appendBuffer(NativeBuffer buffer) {
        int offset = lastEndOffset;
        Node node = Node.of(offset, buffer);
        buffers.add(node);
        lastEndOffset = node.end;
    }

    public byte read(int index) {
        Pair<Integer, Integer> idx = idx(index);
        int nodeIdx = idx.getLeft();
        int byteIdx = idx.getRight();
        return buffers.get(nodeIdx).buf().read(byteIdx);
    }

    private Pair<Integer, Integer> idx(int i) {
        if (i < 0 || i > lastEndOffset) {
            throw new IndexOutOfBoundsException("Index: " + i + ", should be [0, " + lastEndOffset + "]");
        }

        int low = 0;
        int high = buffers.size() - 1;
        while (low <= high) {
            int mid = (low + high) >>> 1;
            Node n = buffers.get(mid);

            if (i >= n.end) {
                low = mid + 1;
            } else if (i <= n.start) {
                high = mid - 1;
            } else {
                return Pair.of(mid, i - n.start);
            }
        }
        throw new IllegalStateException("Should never reach here: " + i);
    }

    private record Node(int start, int end, NativeBuffer buf) {
        static Node of(int offset, NativeBuffer buf) {
            return new Node(offset, offset + buf.getSize(), buf);
        }
    }
}
