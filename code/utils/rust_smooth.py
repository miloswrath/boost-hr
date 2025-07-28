import hr_smooth
import numpy as np
import pandas as pd

hr_df = pd.read_csv('/mnt/lss/Projects/BOOST/InterventionStudy/3-experiment/data/polarhrcsv/Supervised/sub8005/8005_wk4_ses18.CSV', skiprows=2, header=0)
# turn the hr column into a numpy array of floats
hr = hr_df['HR (bpm)'].to_numpy().astype(float)
sm = hr_smooth.median_filter(hr, 9)

hr_df = hr_df.drop(columns=['HR (bpm)'])
hr_df['hr'] = sm

hr_df = hr_df[['hr', 'Time']]
hr_df = hr_df.rename(columns={'Time': 'time'})
hr_df = hr_df[10:]
hr_df.reset_index(drop=True, inplace=True)
hr_df.to_json('../tests/8005_wk8_ses.json', orient='records', date_format='iso')

