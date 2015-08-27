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
			rest.armed_text = rest.status.armed ? "Armed" : "Disarmed";
		}).then(function() {
			setTimeout(reloadStatus, 100);
		});
	}

	function reloadLog() {
		$http.get(API_URL + "/log").success(function(data) {
			rest.logs = data;
		}).then(function() {
			setTimeout(reloadLog, 250);
		});
	}

	function reloadLogMin() {
		$http.get(API_URL + "/log_reduced").success(function(data) {
			rest.logs_min = data;
		}).then(function() {
			setTimeout(reloadLogMin, 250);
		});
	}

	function reloadConfig() {
		$http.get(API_URL + "/config").success(function(data) {
			rest.config = data;
		});
	}

	reloadStatus();
	reloadLog();
	reloadLogMin();
	reloadConfig();

	rest.arm = function(cb) {
		$http.get(API_URL + "/arm").success(function(data) {
			cb(data);
		});
	}

	rest.disarm = function(cb) {
		$http.get(API_URL + "/disarm").success(function(data) {
			cb(data);
		});
	}

	rest.motor_test = function(cb) {
		$http.get(API_URL + "/motor_test").success(function(data) {
			cb(data);
		});		
	}

	return rest;
});