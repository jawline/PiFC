'use strict';

angular.module('RestServices', []).factory('$restService', function($http) {
	var rest = {
		status: {}
	};

	function reloadStatus() {
		$http.get(API_URL + "/status").success(function(data) {
			rest.status = data;
			setTimeout(reloadStatus, 50);
		});
	}

	reloadStatus();

	return rest;
});