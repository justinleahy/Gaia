import React, { useEffect, useState } from 'react';

interface Health {
    CurrentTime: string;
}

const formatToLocalTime = (isoString: string) => {
    const date = new Date(isoString);

    const formatter =  new Intl.DateTimeFormat('en-US', {
        month: 'long',
        day: 'numeric',
        year: 'numeric',
        hour: 'numeric',
        minute: '2-digit',
        second: '2-digit',
        hour12: true,
        timeZoneName: 'short'
    });

    return formatter.format(date);
};

const HealthCheck: React.FC = () => {
    const [currentTime, setCurrentTime] = useState<string>('');

    useEffect(() => {
        const fetchHealth = async () => {
            try {
                const response = await fetch('http://localhost:8080/api/health');
                if(!response.ok) {
                    throw new Error(`HTTP error! Status: ${response.status}`);
                }

                const data: Health = await response.json();
                setCurrentTime(formatToLocalTime(data.CurrentTime));
            } catch (error) {
                console.error("Failed to fetch health check:", error);
            }
        };

        fetchHealth();
        const interval = setInterval(fetchHealth, 1000);
        return () => clearInterval(interval);
    }, []);

    return (
        <div>
            <h1>API Health Check</h1>
            <p><strong>Current Time:</strong> {currentTime || 'Loading...' }</p>
        </div>
    );
};

export default HealthCheck;