package zone.hwj.vita.def;

import java.io.InputStream;
import java.util.ArrayList;
import java.util.List;
import reactor.core.publisher.Flux;
import reactor.core.publisher.Mono;
import zone.hwj.vita.tools.pair.Pair;

public class NativeBufferGroup extends InputStream {
    private final List<Node> buffers = new ArrayList<>();
    private volatile int count = 0;
    private int pos = 0;

    private Node lastAccessed;

    private NativeBufferGroup(NativeBuffer... init) {
        for (NativeBuffer buffer : init) {
            appendBuffer(buffer);
        }
    }

    public static NativeBufferGroup of(NativeBuffer... buffers) {
        return new NativeBufferGroup(buffers);
    }

    public static Mono<NativeBufferGroup> of(Flux<NativeBuffer> buffers) {
        return buffers.collect(NativeBufferGroup::new, NativeBufferGroup::appendBuffer);
    }

    public synchronized void appendBuffer(NativeBuffer buffer) {
        int offset = count;
        Node node = Node.of(offset, buffer);
        buffers.add(node);
        count = node.end();
    }

    public int readAt(int index) {
        if (lastAccessed != null && index >= lastAccessed.start() && index < lastAccessed.end()) {
            return lastAccessed.buf().read(index - lastAccessed.start());
        }
        Pair<Integer, Integer> idx = idx(index);
        int nodeIdx = idx.getLeft();
        int byteIdx = idx.getRight();
        Node node = buffers.get(nodeIdx);
        lastAccessed = node;
        return node.buf().read(byteIdx) & 0xff;
    }

    private Pair<Integer, Integer> idx(int i) {
        if (i < 0 || i > count) {
            throw new IndexOutOfBoundsException("Index: " + i + ", should be [0, " + count + "]");
        }

        int low = 0;
        int high = buffers.size() - 1;
        while (low <= high) {
            int mid = (low + high) >>> 1;
            Node n = buffers.get(mid);

            if (i >= n.end) {
                low = mid + 1;
            } else if (i < n.start) {
                high = mid - 1;
            } else {
                return Pair.of(mid, i - n.start);
            }
        }
        throw new IllegalStateException("Should never reach here: " + i);
    }

    @Override
    public int read() {
        return pos < count ? readAt(pos++) : -1;
    }

    @Override
    public synchronized int available() {
        return count - pos;
    }

    private record Node(int start, int end, NativeBuffer buf) {
        static Node of(int offset, NativeBuffer buf) {
            return new Node(offset, offset + buf.getSize(), buf);
        }
    }
}
