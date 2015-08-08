'use strict';

angular.module('RestServices', []).factory('$restService', function($http) {
	var rest = {
		status: {},
		config: {},
		logs: ""
	};

	function reloadStatus() {
		$http.get(API_URL + "/status").success(function(data) {
			rest.status = data;
			setTimeout(reloadStatus, 100);
		});
	}

	function reloadLog() {
		$http.get(API_URL + "/log").success(function(data) {
			rest.logs = data;
			setTimeout(reloadLog, 250);
		});
	}

	function reloadConfig() {
		$http.get(API_URL + "/config").success(function(data) {
			rest.config = data;
		});
	}

	reloadStatus();
	reloadLog()
	reloadConfig();

	return rest;
});