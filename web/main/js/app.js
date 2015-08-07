'use strict';

/* App Module */

var imcluelessSite = angular.module('imcluelessSite', [ 'ngRoute' ]);

imcluelessSite.config([ '$routeProvider', function($routeProvider) {
	$routeProvider.when('/', {
		redirectTo : '/landing'
	}).when('/landing', {
		templateUrl : 'partials/landing.html',
		controller : 'LandingCtrl'
	}).when('/error404', {
		templateUrl : 'partials/error404.html'
	}).when('/logs', {
		templateUrl : 'partials/logs.html'
	}).when('/status', {
		templateUrl : 'partials/status.html'
	}).otherwise({
		redirectTo : '/error404'
	});
}]).run(function ($rootScope) {});
