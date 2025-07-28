import pandas as pd
import numpy as np
import json
import os

class Preprocessor:

    def __init__(self, supervised_path, unsupervised_path):
        self.supervised_path = supervised_path
        self.unsupervised_path = unsupervised_path
        self.zones = zones
        return None

    def process_supervised(self):
        '''
            Steps to process supervised data:
            1. Load as df
            2. turn it into workable data by only keeping time and hr columns
        
        '''
        # skip two rows. then first row is header. then only keep time and hr columns
        unclean = pd.read_csv(self.supervised_path, skiprows=2, header=0, usecols=['Time', 'HR (bpm)'])
        unclean.columns = ['time', 'hr']
        if unclean.empty:
            return 1, unclean
        elif unclean.shape[0] < 100:
            return 2, unclean
        else:
            # Convert 'time' to datetime
            unclean['time'] = pd.to_datetime(unclean['time'], format='%H:%M:%S', errors='coerce')
            clean = unclean
            del unclean
            return 0, clean


    def process_unsupervised(self):
        '''
            Steps to process unsupervised data:
            1. Load as df
            2. turn it into workable data by only keeping time and hr columns
            3. remove any rows after t=50 minutes
        '''
        unclean = pd.read_csv(self.unsupervised_path, skiprows=2, header=0, usecols=['Time', 'HR (bpm)'])
        unclean.columns = ['time', 'hr']
        if unclean.empty:
            return 1, unclean
        elif unclean.shape[0] < 100:
            return 2, unclean
        else:
            # Convert 'time' to datetime
            unclean['time'] = pd.to_datetime(unclean['time'], format='%H:%M:%S', errors='coerce')
            # Remove rows after 50 minutes
            unclean = unclean[unclean['time'] <= pd.Timestamp('00:50:00')]
            clean = unclean
            del unclean
            return 0, clean

