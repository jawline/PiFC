'use strict';

var API_PORT = "14320";
var API_URL = location.protocol+'//'+location.hostname + ":" + API_PORT;

/* Controllers */
function LandingCtrl($scope) {}

function CommandsCtrl($scope) {
	$scope.api_url = API_URL;
}

function StatusCtrl($scope, $http) {
	$http.get(API_URL + "/status").success(function(data) {
		$scope.status = data;
	});
}