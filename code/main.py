import pandas as pd
import numpy as np
import os

class HR_QA():

    def __init__(self, days=30, machine='home'):
        self.days = days
        if machine == 'home':
            self.study_path = os.path.join('/mnt/lss/Projects/BOOST/InterventionStudy/3-experiment/data/polarhrcsv')
        elif machine == 'vosslnx':
            self.study_path = os.path.join('/mnt/nfs/lss/vosslabhpc/Projects/BOOST/InterventionStudy/3-experiment/data/polarhrcsv')
        elif machine == 'argon':
            self.study_path = os.path.join('/Shared/vosslabhpc/Projects/BOOST/InterventionStudy/3-experiment/data/polarhrcsv')
        else: raise ValueError("Unknown machine type. Please specify 'home', 'vosslnx', or 'argon'.")

        self.subs = self._get_subject_list()
        for sub in self.subs:
            self.zones = self._get_zones()
            self.supervised_path = os.path.join(self.study_path, 'Supervised', sub)
            self.unsupervised_path = os.path.join(self.study_path, 'Unsupervised', sub)
            self.run_pipeline(self.supervised_path, self.unsupervised_path, self.zones)

        return None


    def run_pipeline(self, supervised_path, unsupervised_path, zones):

        # Preprocess the data
        from utils.preproc import Preprocessor
        from utils.qc import QC
        for file in os.listdir(self.supervised_path):
            path = os.path.join(self.supervised_path, file)
            week = file.split('_')[1][-1]
            P = Preprocessor(path, None)
            code, p_supervised = P.process_supervised()
            # handle code or process p_supervised
            if code != 0:
                print(f"Error processing supervised data for {file}: code {code}")
                continue
            else:
                # Process p_supervised further if needed
                Q = QC(p_supervised, None, zones, week=week)
                Q.qc_supervised(df=p_supervised)

                

        for file in os.listdir(self.unsupervised_path):
            path = os.path.join(self.unsupervised_path, file)
            P = Preprocessor(None, path)
            code, p_unsupervised = P.process_unsupervised()
            # handle code or process p_unsupervised
        # QC the data


        return None
    
    def _get_subject_list(self):
        """
        Get the list of subjects from the study/supervised path - supervised is always done first so if in supervised they will be in unsupervised.
        """
        subject_list = []
        supervised_path = os.path.join(self.study_path, 'supervised')
        for subject in os.listdir(supervised_path):
            if os.path.isdir(os.path.join(supervised_path, subject)):
                subject_list.append(subject)

        return subject_list


    def _get_zones(self, subject=None):
        """
        Get the zones from the zone path.
        """
        if not subject:
            # if no subject is given, return a default value or raise an error
            return 1
        zone_path = os.path.join(self.study_path.split('3-Experiment')[0], '1-projectManagement', 'participants', 'ExerciseSessionMaterials', 'Intervention Materials', 'BOOST HR ranges.xlsx')
        # read some data here blah blah 
        zones = pd.read_excel(zone_path, index_col=0).to_dict(orient='index')
        # keep row with 'BOOST ID' equal to subject
        zones = zones[zones['BOOST ID'] == subject] 
        if zones.empty:
            raise ValueError(f"No zones found for subject {subject}. Please check the subject ID or the zone file.")
        z = zones.iloc[0]
        del zones 
        raw = [
            (z['Zone 1 (55-60%)'], z['Unnamed: 6']),
            (z['Zone 2 (60-65%)'], z['Unnamed: 8']),
            (z['Zone 3 (65-70%)'], z['Unnamed: 10']),
            (z['Zone 4 (70-75%)'], z['Unnamed: 12']),
            (z['Zone 5 (75-80%)'], z['Unnamed: 14']),
        ]
        del z
        zones = [{'zone': i+1, 'low': low, 'high': high} for i, (low, high) in enumerate(raw)]
        def fix_midpoint_snap_integer(zones):
            fixed = [zones[0].copy()]
            for i in range(1, len(zones)):
                prev = fixed[-1]
                curr = zones[i].copy()
                m = round((prev['high'] + curr['low']) / 2)
                prev['high'] = m
                curr['low'] = m
                fixed.append(curr)

            # Final rounding and zone reformat
            for z in fixed:
                z['low'] = int(round(z['low']))
                z['high'] = int(round(z['high'])) - 1  # close interval at high

            # Add Zone 0 (before Zone 1)
            zone0 = {
                'zone': 0,
                'low': 0,
                'high': fixed[0]['low'] - 1
            }

            # Add Zone 6 (after Zone 5)
            zone6 = {
                'zone': 6,
                'low': fixed[-1]['high'] + 1,
                'high': float('inf')  # can be used as open upper bound
            }

            # Add zone numbers to existing fixed zones
            for i, z in enumerate(fixed):
                z['zone'] = i + 1

            return [zone0] + fixed + [zone6]        
        zones = fix_midpoint_snap_integer(zones)


        return zones

