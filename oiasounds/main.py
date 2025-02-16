import librosa
from librosa import display
import numpy as np
import pydub as pd
from pydub import AudioSegment as As
from pydub.playback import play
import matplotlib.pyplot as plt

def everything(file_path):
    last_8_seconds = As.from_file(file_path, format="mp3")[-7000:]
    last_8_seconds.export("final.mp3", format="mp3")
    
    
def get_frequency_time_array(audio_file):
    y, sr = librosa.load(audio_file, sr=None)  
    
   
    D = np.abs(librosa.stft(y))
    

    frequencies = librosa.fft_frequencies(sr=sr)
    times = librosa.frames_to_time(np.arange(D.shape[1]), sr=sr)
    

    frequency_time_array = list(zip(times, np.mean(D, axis=0)))
    
    return frequency_time_array





frequency_time_data = get_frequency_time_array("segment_0.mp3")
print(frequency_time_data)