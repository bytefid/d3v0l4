from ..numeric import F32


def raster_to_sense(popola_config, game_config, raster_buffer_curr, raster_buffer_prev,
                    sense_buffer_on, sense_buffer_off):
    size = game_config.get_raster_size()
    block = popola_config.get_sense_block_size()
    blocks_per_row = size // block
    total = blocks_per_row ** 2
    assert len(sense_buffer_on) >= total
    assert len(sense_buffer_off) >= total
    pixels = F32(block * block)
    sense_buffer_on[:total] = [0.0] * total
    sense_buffer_off[:total] = [0.0] * total
    changed_blocks = set()
    covered = blocks_per_row * block
    # Most pixels are unchanged. Skip zero additions while retaining the exact
    # row/column summation order within each block and f32 rounding on changes.
    for py in range(covered):
        pixel_row = py * size
        block_row = (py // block) * blocks_per_row
        for px in range(covered):
            pixel = pixel_row + px
            current, previous = raster_buffer_curr[pixel], raster_buffer_prev[pixel]
            if current == previous:
                continue
            delta = F32(current) - previous
            index = block_row + px // block
            if delta > 0.0:
                sense_buffer_on[index] += delta
            elif delta < 0.0:
                sense_buffer_off[index] += -delta
            changed_blocks.add(index)
    for index in changed_blocks:
        sense_buffer_on[index] /= pixels
        sense_buffer_off[index] /= pixels
