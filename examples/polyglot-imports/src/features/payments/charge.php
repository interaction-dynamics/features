<?php
require_once "../ledger/ledger.php";

function charge($amount) {
    return record_entry($amount);
}
