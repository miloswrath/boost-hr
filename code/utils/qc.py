#imports go here 
from qc_utils import QCUtils

class QC:

    def __init__(self, supervised, unsupervised, zones, week):
        self.supervised = supervised
        self.unsupervised = unsupervised
        self.zones = zones
        self.week = week

        self.rules = {
            '1': {
                'zones': [1, 2],
                'percenthrmax': [55, 70],
                'boundedminutes': 15,
            },
            '2': {
                'zones': [1, 2],
                'percenthrmax': [55, 70],
                'boundedminutes': 20,
            },
            '3': {
                'zones': [2],
                'percenthrmax': [60, 70],
                'boundedminutes': 25,
            },
            '4': {
                'boundedminutes': 30,
            },
            '5': {
                'zones': [2, 3],
                'percenthrmax': [65, 75],
                'boundedminutes': 30,
            },
            '6': {
                'zones': [2, 3],
                'percenthrmax': [65, 75],
                'boundedminutes': 30,
            },
            '7': {
                'zones': [2, 3],
                'percenthrmax': [65, 75],
                'boundedminutes': 30,
            },
            '8': {
                'zones': [2, 3],
                'percenthrmax': [65, 75],
                'boundedminutes': 30,
            },
            '9': {
                'zones': [2, 3],
                'percenthrmax': [65, 75],
                'boundedminutes': 30,
            },
            '10': {
                'zones': [2, 3],
                'percenthrmax': [65, 80],
                'boundedminutes': 30,
            },
            '11': {
                'zones': [3],
                'percenthrmax': [70, 80],
                'boundedminutes': 30,
            },
            '12': {
                'zones': [3],
                'percenthrmax': [70, 80],
                'boundedminutes': 30,
            },
        }
        self.utils = QCUtils(self.supervised, self.unsupervised, self.zones, self.week)




    def qc_supervised(self, df):
        """
        Perform quality control on supervised data.
        1. extended time spent in zone 6? greater than 10 minutes
        2. extended time spent in zones less than minimum required for that zone? greater than 5 minutes excluding first and last 5 minutes
        3. were the session goals met?
        """
        extended_time_in_zone_6 = self.utils._above_6(df)


        return df


