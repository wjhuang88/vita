package zone.hwj.vita.tools;

import java.io.IOException;
import java.io.OutputStream;
import java.lang.foreign.ValueLayout;
import reactor.core.publisher.FluxSink;
import zone.hwj.vita.def.NativeBuffer;

public class NativeBufferOutputStream extends OutputStream {
	private NativeBuffer buffer;
	private volatile boolean closed = false;
	private int writerIndex = 0;
	private final Object closeLock = new Object();

	private final SlicedBufferPool factory;
	private final FluxSink<NativeBuffer> emitter;
	private final int bufferSize;

	public NativeBufferOutputStream(SlicedBufferPool factory, FluxSink<NativeBuffer> emitter) {
		this.factory = factory;
		this.emitter = emitter;
		this.bufferSize = factory.getBlockSize();
		buffer = factory.acquire(this.bufferSize).getFirst();
	}

	@Override
	public void write (int b) throws IOException {
        if (writableByteCount() <= 0) {
            flush();
        }
		buffer.getDataPtr().set(ValueLayout.JAVA_BYTE, this.writerIndex, (byte) b);
		this.writerIndex++;
	}

	@Override
	public void flush() {
		emitter.next(factory.resize(buffer, writerIndex));
		if (!closed) {
			buffer = factory.acquire(bufferSize).getFirst();
			this.writerIndex = 0;
		}
	}

	@Override
	public void close() {
		if (closed) {
			return;
		}
		synchronized (closeLock) {
			if (closed) {
				return;
			}
			closed = true;
		}
		flush();
		emitter.complete();
	}

	private int writableByteCount() {
		return buffer.getSize() - this.writerIndex;
	}
}