import librosa
import numpy as np
from pydub import AudioSegment
from pydub.silence import split_on_silence

def separate_unique_sounds(audio_file):

    audio = AudioSegment.from_file(audio_file)
    
    chunks = split_on_silence(audio, min_silence_len=3, silence_thresh=-40)
    

    segmented_files = []
    for i, chunk in enumerate(chunks):
        filename = f"unique_sound_{i}.mp3"
        chunk.export(filename, format="mp3")
        segmented_files.append(filename)
    
    return segmented_files


segmented_files = separate_unique_sounds("segment_0.mp3")
print(segmented_files)
