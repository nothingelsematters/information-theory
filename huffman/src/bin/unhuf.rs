use huffman::byte_processor::ByteProcessor;

fn main() {
    huffman::utils::main(&huffman::decode::Decoder::write_processed);
}
