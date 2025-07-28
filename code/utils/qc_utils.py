
class QCUtils:

    def __init__(self, zones, rules, week, supervised=None, unsupervised=None):
        self.supervised = supervised
        self.unsupervised = unsupervised
        self.zones = zones
        self.rules = rules
        self.week = week


    def _above_6(self, df):
        # check if there is extended time spent in zone 6 look for greater than 10 minutes in seconds
        zone_6_min = self.zones['6']['low']
        # if hr in df is greater than min of seconds in zone 6 for more than 10 minutes
        time_in_zone_6_seconds = (df['hr'] > zone_6_min).sum()
        # Check if accumulated time in Zone 6 exceeds 10 minutes (600 seconds)
        extended_time_in_zone_6 = time_in_zone_6_seconds >= 600
        return extended_time_in_zone_6


    def _extract_bounded_minutes(self, initial_minutes=5):
        """
        Extract data after excluding the first specified minutes.
        
        Args:
            initial_minutes (int): Number of initial minutes to exclude. Default is 5.
            
        Returns:
            DataFrame: Filtered data after the specified initial minutes
        """
        # Convert minutes to seconds
        initial_seconds = initial_minutes * 60
        
        # Make a copy of the supervised data
        df = self.supervised.copy()
        
        # Filter out data from the first 'initial_seconds'
        bounded_data = df[df['time'] > initial_seconds]
        
        return bounded_data


    def _return_rule_data(self):
        """
        Return the rule data for a given rule number.
        """
        rules = self.rules.get(self.week, {})
        # using the returned bounded minutes data - extract the bounded minutes after the first 5 minutes
        df = self._extract_bounded_minutes(5)
        
        # Apply rules to the bounded data
        result = {}
        for rule_id, rule_config in rules.items():
            # Process according to rules (actual implementation depends on rule structure)
            result[rule_id] = self._process_rule(df, rule_config)
        
        return result
                


    def _process_rule(self, df, rule_config):
        """
        Process the DataFrame according to the given rule configuration.
        
        Args:
            df (DataFrame): The DataFrame to process.
            rule_config (dict): The configuration for the rule.
            
        Returns:
            dict: Processed results based on the rule configuration.
        """
        # Placeholder for actual processing logic
        # This should include checks against zones, percenthrmax, and boundedminutes
        processed_result = {
            'zones': rule_config.get('zones', []),
            'percenthrmax': rule_config.get('percenthrmax', []),
            'boundedminutes': rule_config.get('boundedminutes', 0)
        }
        
        return processed_result



