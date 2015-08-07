'use strict';

angular.module('RestServices', []).factory('$restService', function($http) {
	var rest = {
		status: {},
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

	reloadStatus();
	reloadLog()

	return rest;
});